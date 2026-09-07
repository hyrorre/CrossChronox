import type { scores } from '../../db/schema'
import type { IrScoreSubmission } from '../../../shared/types/ir'
import { playedAtDate, scoreSubmissionMetadata } from './common'
import { stableStringify } from './verification'

export class IrIdempotencyCollisionError extends Error {
  constructor() {
    super('idempotency key collision: submission differs from the existing score')
  }
}

function normalizedPlayOptions(options: Record<string, unknown>) {
  const normalized = { ...options }
  for (const key of ['seed', 'random_seed']) {
    const value = normalized[key]
    if (
      (typeof value === 'number' && Number.isSafeInteger(value)) ||
      (typeof value === 'string' && /^-?\d+$/.test(value))
    ) {
      normalized[key] = BigInt(value).toString()
    }
  }
  return normalized
}

// Compare persisted gameplay data, excluding evidence (regenerated on retry),
// client version and mutable chart metadata. No historical fingerprint migration is needed.
export function assertIdempotentSubmission(
  existing: typeof scores.$inferSelect,
  playerId: string,
  payload: IrScoreSubmission,
) {
  const { doubleOption, appliedDoubleOption, sourceKind } = scoreSubmissionMetadata(
    payload.play_options,
  )
  const expected = {
    playerId,
    chartSha256: payload.chart.sha256,
    playMode: payload.rule.play_mode,
    keyMode: payload.rule.key_mode,
    gauge: payload.rule.gauge,
    lnPolicy: payload.rule.ln_policy,
    effectiveLnMode: payload.rule.effective_ln_mode,
    ruleMode: payload.rule.rule_mode,
    judgeAlgorithm: payload.rule.judge_algorithm,
    scoring: payload.rule.scoring,
    clearType: payload.result.clear,
    playedAt: playedAtDate(payload.result.played_at)?.getTime() ?? null,
    durationMs: payload.result.duration_ms ?? null,
    judges: payload.result.judges,
    exScore: payload.result.ex_score,
    avgJudgeMs: payload.result.avg_judge_ms ?? null,
    maxCombo: payload.result.max_combo,
    notes: payload.result.notes,
    passNotes: payload.result.pass_notes ?? payload.result.notes,
    minBp: payload.result.min_bp,
    minCb: payload.result.min_cb,
    deviceType: payload.play_options.device_type,
    doubleOption,
    appliedDoubleOption,
    sourceKind,
    playOptions: normalizedPlayOptions({
      ...payload.play_options,
      double_option: doubleOption,
      applied_double_option: appliedDoubleOption,
      source_kind: sourceKind,
    }),
    replayHash: payload.replay?.hash ?? null,
    replayFormat: payload.replay?.format ?? null,
    replayUploadIntent: payload.replay?.upload_intent ?? null,
  }
  const actual = {
    ...existing,
    playedAt: existing.playedAt?.getTime() ?? null,
    playOptions: normalizedPlayOptions({
      ...existing.playOptions,
      double_option: existing.doubleOption,
      applied_double_option: existing.appliedDoubleOption,
      source_kind: existing.sourceKind,
    }),
  }
  const differs = Object.keys(expected).some((name) => {
    const key = name as keyof typeof expected
    return stableStringify(actual[key]) !== stableStringify(expected[key])
  })
  if (differs) {
    console.warn('IR idempotency key collision', {
      playerId,
      idempotencyKey: payload.idempotency_key,
      existingScoreId: existing.id,
      existingChartSha256: existing.chartSha256,
      incomingChartSha256: payload.chart.sha256,
    })
    throw new IrIdempotencyCollisionError()
  }
}
