# Frame pacing と計測

## 待機方式

描画周期は `FramePacer` の monotonic deadline で管理する。通常の起床ジッターでは
deadline 列を維持する。遅れを取り戻すため次の間隔が周期の半分未満になる場合は、
現在時刻から再開して長短フレームの連発を防ぐ。スキン更新の即時描画は周期を変更せず、
FPS・focus・実効 present mode・window mode が変わった場合は周期をリセットする。

Windows の focused Immediate・FPS制限ありの場合だけ、期限の
`min(600us, frame_budget / 8)` 前に winit の `WaitUntil` で起床する。
RedrawRequested 到着後、残りがこの範囲に収まる場合のみ render thread で短く待つ。
winit 0.30.13 自体がWindowsの高精度waitable timerを使っているため、同じtimerを
二重に追加しない。600usは実測した約0.5〜0.6msのwake p95を踏まえた上限であり、
OSによる長いpreemptionを防ぐものではない。

FIFO / Mailbox / background / Unlimited / Windows以外ではこの短時間待機を使わない。
設定上のImmediateが別modeへfallbackした場合も、surfaceの実効modeで判断する。
待機中にgameplay/input/audio/GPUのlockは保持せず、GameplayRuntimeのwakeは変更しない。
追加のCPU実行時間は必要になるが、長い待機はOSへ返す。通常のperiodic frameで
短時間待機に使う上限は1周期の1/8、120FPSでは600us、240FPSでは約521usとなる。

これはdisplay/compositorとの位相同期を実装したものではない。VSync、VRR、driver設定や
present queue設定は変更しない。ImmediateのtearingやGPU stallを解消する保証もない。

## 診断

```powershell
$env:RUST_LOG = 'info,bmz_player::play_profile=debug,bmz_player::frame_pacing=debug'
```

`CPU frame cadence` は描画開始間隔と、理想周期からの絶対誤差を5秒ごとに集計する。
`snapshot cadence` は描画時刻へ投影したsnapshotのage、前回描画との時刻差、
同じ描画時刻を使った回数を集計する。`publication_age_us` は投影前のgameplay状態の古さで、
描画時刻のageとは別に記録する。histogramのp95/p99はbucket上限の近似値。
`bmz_player::frame_pacing=trace` にすると個々のsampleを記録でき、offlineで正確な分位を計算できる。
通常プレイでTRACEは必要ない。既存profilerのwake latenessはOSへ渡した早期wake期限に
対する遅れなので、この変更後の描画期限の遅れそのものとは区別する。

FPS表示とアプリのpresent間隔だけでは実際の表示間隔を証明できない。
[PresentMon](https://github.com/GameTechDev/PresentMon) の対象process限定CSVと併用する。
Vulkanで取得できない値（NA等）はゼロ遅延として扱わない。Composed Flipから
Independent Flipへの遷移、focus喪失、ロード中を分け、計測中は他のbuildを走らせない。

設定の比較には `BMZ_DATA_DIR` / `BMZ_LOGS_DIR` を作業用ディレクトリへ向け、
`BMZ_RESOURCE_DIR` だけ同梱dataを参照させる。普段のconfig/score DBを上書きしない。
既存configからコピーする場合はOBS・IR等の外部連携を検証用環境へ持ち込まない。

## 2026-09-08 の実測

Windows、RTX 5090、DX12、3840x2160 Native、borderless、実効Immediate、frame latency 1。
同梱sample / default skin / autoplayをreleaseで実行。修正前は計測追加のみの
`c6ad9245`、修正後は`0a1150cf`。PresentMon 2.5.1を双方で併用した。
GPUに余裕がある環境での短時間測定であり、GTX 1660等の報告環境の再現ではない。

| FPS制限 | 描画開始間隔の誤差 avg 前→後 | 同 p99 前→後 | PresentMon表示間隔 p99 前→後 |
|---|---:|---:|---:|
| 120 | 220 → 27 us | 730 → 417 us | 9.774 → 9.531 ms |
| 240 | 201 → 46 us | 686 → 454 us | 5.709 → 5.319 ms |

CPU cadenceはgameplay snapshot consume開始から1秒を除外した約14秒、
PresentMonはIndependent Flipの先頭2秒と末尾1秒を除外した約12秒を集計。
表示間隔はtearing有効のIndependent Flip更新間隔であり、完全な1枚のscanoutを
数えた値ではない。小さい表示側の差は反復測定や別環境での確認が必要。
120FPSの実行全体のCPU時間は4.25→3.75秒、240FPSは5.55→6.95秒だった。
起動・ロードも含むためCPU時間の減少を最適化の成果と解釈しない。

報告された持続的な30/60FPS相当のカクつきは、この条件では修正前にも再現しなかった。
この測定時点ではsnapshot ageが120FPSで約7ms、240FPSで約2.8ms残っており、
snapshotの時刻に固定されたノーツ位置には別の周期の揺れが残っていた。
この表はCPU待機処理の比較であり、gameplay分離前の `e974998e` に対する
ノーツの滑らかさの改善を示すものではない。

## 描画時刻でのノーツ投影

ノーツ位置は最新のimmutableな描画用状態から、consumerのAudioClock時刻で計算する。
gameplayの公開周期やrendererとの位相差に座標更新周期を固定しない。
描画時刻の単調性はconsumer内で保持し、seek/retryの新しいgenerationでリセットする。
FPS制限、present mode、gameplay wake、判定・音声の処理はこの変更の対象外。

自動テストは120/240FPS、5種類の公開位相、0/16/33/100/250msの描画停止を組み合わせ、
等速区間の座標と移動量を数式に照合する。さらに、公開データを受け取れないconsumer、
新たに可視範囲へ入るノーツ・Mine・LN、STOP、BPM/SCROLL/SPEED、CONSTANT、
PMS/retention、LN/CN/HCN状態、表示offsetの変更とclock resetを検証する。
既存のruntimeテストは、描画停止の有無で判定・replay・score・audio schedulingが
従来のsession advanceと一致することを引き続き確認する。

### 投影変更の実測

`604ff9d2` と `301c6b79` のrelease buildを、同じ
`target/release/bmz-player.exe` パスで比較した。Windows / RTX 5090 / DX12 / 4K Native /
borderless / Immediate / frame latency 1、同梱sample・default skin・autoplay。
各runでPresentMonを併用し、Playのsnapshot sampleの先頭・末尾1秒を除外した約13秒を集計。

| FPS制限 | 描画間隔と投影時刻差のずれ p99 前→後 | 理想周期と投影時刻差のずれ p99 前→後 |
|---|---:|---:|
| 120 | 2091 → 203 us | 1990 → 420 us |
| 240 | 2147 → 182 us | 2114 → 474 us |

前者は `abs(snapshot時刻の差分 - CPU描画開始間隔)`、後者は
`abs(snapshot時刻の差分 - 1/FPS)`。描画開始とclock取得は異なる位置なので、前者にも
描画開始後のCPU処理時間の揺れが含まれる。修正後のpublication ageは平均約7.17ms / 2.85ms
だが、ノーツ座標の時刻にはその遅れを持ち込まない。判定状態を先読みするものではない。
この計測はCPU側の描画時刻の追従を示すもので、GPU/presentや物理scanoutのstall解消を
示すものではない。

同じDX12/4K条件のECFNスキンでも、VSync（実効Fifo）とFastVSync（実効Mailbox）の
Unlimited設定でsample autoplayをResultまで実行した。5秒ごとの集計では投影後ageと
同一時刻の繰り返しはいずれも0だった。FPS半減の調査はこの修正の対象外。
判定・replay仕様を変えて描画の不連続を隠さない。

Vulkanでも同条件の最終ビルドを計測し、描画間隔の誤差は120FPSでavg 30us / p99 405us、
240FPSでavg 48us / p99 442usだった。Vulkanの修正前比較は行っていない。

## 回帰テスト

`cargo test -p bmz-player frame_runtime` で次を検証する。

- 120/240FPSの待機区間と短時間待機の上限
- FIFO / Mailbox / background / Unlimited / 他OSで短時間待機しないこと
- 即時描画を連続して要求しても周期が変わらないこと
- 16/33/100/250ms遅延から復帰しても極端な短間隔を連発しないこと
- FPS切替、通常のdeadline維持、wakeの統計、FPSカウンターの既存動作

gameplayの独立性は既存の `gameplay_runtime::tests` とworkspace testでも検証する。

実行結果: `cargo fmt --check`、`cargo clippy --workspace --all-targets --features experimental-gameinput`
成功、`cargo test --workspace` は3250 passed / 7 ignored。
追加histogramで既存のWinitApp stack上限テストが失敗したため、統計を作成時に確保する
Boxへ移して修正した。再実行中に既存file loggerテストが一度だけファイル数不一致で
失敗した。PIDを再利用した一時ディレクトリに8/27のログが残り、今回のログと2個になった
ことを確認した。単独実行と最後のworkspace全体実行は成功した。
