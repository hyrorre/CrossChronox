import { and, asc, desc, eq, sql, type SQL } from 'drizzle-orm'
import { db, schema } from 'hub:db'

export async function fetchCourseRankingRows(conditions: SQL[], limit: number) {
  const scores = schema.courseScores
  const ranked = db
    .select({
      player_id: scores.playerId,
      course_score_id: scores.id,
      ex_score: scores.exScore,
      clear_type: scores.clearType,
      clear_rank: scores.clearRank,
      course_clear: scores.courseClear,
      max_combo: scores.maxCombo,
      bp: scores.bp,
      device_type: scores.deviceType,
      rule_mode: scores.ruleMode,
      played_at: scores.playedAt,
      server_received_at: scores.serverReceivedAt,
      verification: scores.verification,
      player_position: sql<number>`row_number() over (partition by ${scores.playerId}
      order by ${scores.exScore} desc, ${scores.clearRank} desc, ${scores.bp} asc,
      ${scores.maxCombo} desc, ${scores.serverReceivedAt} desc, ${scores.id} asc)`.as(
        'player_position',
      ),
    })
    .from(scores)
    .where(and(...conditions))
    .as('ranked')
  return db
    .select()
    .from(ranked)
    .where(eq(ranked.player_position, 1))
    .orderBy(
      desc(ranked.ex_score),
      desc(ranked.clear_rank),
      asc(ranked.bp),
      desc(ranked.max_combo),
      desc(ranked.server_received_at),
      asc(ranked.course_score_id),
    )
    .limit(limit)
}
