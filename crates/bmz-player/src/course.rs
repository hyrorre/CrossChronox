use anyhow::{Result, bail};
use bmz_core::course::{CourseConstraints, CourseDefinition, CourseEntry, CourseTrophy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BeatorajaCourseFile {
    One(BeatorajaCourse),
    Many(Vec<BeatorajaCourse>),
}

pub(crate) const LOCAL_COURSE_MAX_ENTRIES: usize = 10;

#[derive(Debug, Deserialize)]
struct BeatorajaCourse {
    #[serde(default)]
    name: String,
    /// Standard beatoraja course format: array of song objects with title/md5/sha256.
    #[serde(default, alias = "song")]
    hash: Vec<BeatorajaCourseSong>,
    /// bmstable header format used by jbmstable-parser.
    #[serde(default)]
    charts: Vec<BeatorajaCourseSong>,
    /// Stella/table-embedded format: flat array of MD5 hex strings.
    #[serde(default, rename = "md5")]
    md5_list: Vec<String>,
    #[serde(default)]
    constraint: Vec<String>,
    #[serde(default)]
    trophy: Vec<BeatorajaTrophy>,
    #[serde(default = "default_release")]
    release: bool,
}

#[derive(Debug, Deserialize)]
struct BeatorajaCourseSong {
    #[serde(default)]
    title: String,
    #[serde(default)]
    md5: String,
    #[serde(default)]
    sha256: String,
}

#[derive(Debug, Deserialize)]
struct BeatorajaTrophy {
    #[serde(default)]
    name: String,
    #[serde(default)]
    missrate: f32,
    #[serde(default)]
    scorerate: f32,
}

#[derive(Debug, Serialize)]
struct ExportedBeatorajaCourse<'a> {
    name: &'a str,
    hash: Vec<ExportedBeatorajaCourseSong<'a>>,
    constraint: Vec<&'static str>,
    trophy: Vec<ExportedBeatorajaTrophy<'a>>,
    release: bool,
}

#[derive(Debug, Serialize)]
struct ExportedBeatorajaCourseSong<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    md5: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct ExportedBeatorajaTrophy<'a> {
    name: &'a str,
    missrate: f32,
    scorerate: f32,
}

fn default_release() -> bool {
    true
}

pub fn parse_beatoraja_course_json(source: &str, json: &str) -> Result<Vec<CourseDefinition>> {
    let file: BeatorajaCourseFile = serde_json::from_str(json)?;
    let courses = match file {
        BeatorajaCourseFile::One(course) => vec![course],
        BeatorajaCourseFile::Many(courses) => courses,
    };

    courses
        .into_iter()
        .enumerate()
        .map(|(index, course)| convert_beatoraja_course(source, index, course))
        .collect()
}

pub(crate) fn parse_beatoraja_course_value(
    source: &str,
    index: usize,
    value: serde_json::Value,
) -> Result<CourseDefinition> {
    let course: BeatorajaCourse = serde_json::from_value(value)?;
    convert_beatoraja_course(source, index, course)
}

pub(crate) fn next_local_course_key<'a>(keys: impl IntoIterator<Item = &'a str>) -> String {
    let keys = keys.into_iter().collect::<std::collections::HashSet<_>>();
    (1..)
        .map(|index| format!("local-course-{index}"))
        .find(|candidate| !keys.contains(candidate.as_str()))
        .expect("course key sequence is finite in practice")
}

pub fn serialize_beatoraja_course_json(courses: &[CourseDefinition]) -> Result<String> {
    let exported = courses
        .iter()
        .map(|course| ExportedBeatorajaCourse {
            name: &course.title,
            hash: course
                .entries
                .iter()
                .map(|entry| ExportedBeatorajaCourseSong {
                    title: &entry.title_hint,
                    md5: entry.md5.as_deref(),
                    sha256: entry.sha256.as_deref(),
                })
                .collect(),
            constraint: exported_local_constraint_names(&course.constraints),
            trophy: course
                .trophies
                .iter()
                .map(|trophy| ExportedBeatorajaTrophy {
                    name: &trophy.name,
                    missrate: trophy.max_miss_rate,
                    scorerate: trophy.min_score_rate,
                })
                .collect(),
            release: course.release,
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&exported).map_err(Into::into)
}

fn exported_local_constraint_names(constraints: &CourseConstraints) -> Vec<&'static str> {
    use bmz_core::course::{
        CourseClassConstraint, CourseGaugeConstraint, CourseJudgeConstraint, CourseLnConstraint,
        CourseSpeedConstraint,
    };
    let mut names = Vec::new();
    match constraints.class {
        CourseClassConstraint::None => {}
        CourseClassConstraint::Grade => names.push("CLASS"),
        CourseClassConstraint::GradeMirrorAllowed => names.push("MIRROR"),
        CourseClassConstraint::GradeRandomAllowed => names.push("RANDOM"),
    }
    if constraints.speed == CourseSpeedConstraint::NoSpeed {
        names.push("NO_SPEED");
    }
    match constraints.judge {
        CourseJudgeConstraint::Normal => {}
        CourseJudgeConstraint::NoGood => names.push("NO_GOOD"),
        CourseJudgeConstraint::NoGreat => names.push("NO_GREAT"),
    }
    match constraints.gauge {
        CourseGaugeConstraint::Default => {}
        CourseGaugeConstraint::Lr2 => names.push("GAUGE_LR2"),
        CourseGaugeConstraint::Keys5 => names.push("GAUGE_5KEYS"),
        CourseGaugeConstraint::Keys7 => names.push("GAUGE_7KEYS"),
        CourseGaugeConstraint::Keys9 => names.push("GAUGE_9KEYS"),
        CourseGaugeConstraint::Keys24 => names.push("GAUGE_24KEYS"),
    }
    match constraints.ln {
        CourseLnConstraint::Default => {}
        CourseLnConstraint::Ln => names.push("LN"),
        CourseLnConstraint::Cn => names.push("CN"),
        CourseLnConstraint::Hcn => names.push("HCN"),
    }
    names
}

pub fn normalize_course_definition(definition: &mut CourseDefinition) {
    definition.kind = CourseDefinition::derive_kind_from_constraints(&definition.constraints);
    definition.constraints.source_constraints =
        definition.constraints.canonical_names().into_iter().map(str::to_string).collect();
}

fn convert_beatoraja_course(
    source: &str,
    index: usize,
    course: BeatorajaCourse,
) -> Result<CourseDefinition> {
    let BeatorajaCourse { name, hash, charts, md5_list, constraint, trophy, release } = course;
    // Build entries from the local-course `hash` object format, the bmstable
    // `charts` object format, or the older flat `md5` string format.
    let object_entries = if !hash.is_empty() { hash } else { charts };
    let entries: Vec<CourseEntry> = if !object_entries.is_empty() {
        object_entries
            .into_iter()
            .enumerate()
            .map(|(entry_index, song)| CourseEntry {
                title_hint: if song.title.trim().is_empty() {
                    format!("course {}", entry_index + 1)
                } else {
                    song.title
                },
                md5: normalize_hash(song.md5, 32),
                sha256: normalize_hash(song.sha256, 64),
                chart_id: None,
            })
            .collect()
    } else if !md5_list.is_empty() {
        // Stella/table format: md5 is a flat array of hex strings.
        md5_list
            .into_iter()
            .enumerate()
            .map(|(entry_index, md5)| CourseEntry {
                title_hint: format!("course {}", entry_index + 1),
                md5: normalize_hash(md5, 32),
                sha256: None,
                chart_id: None,
            })
            .collect()
    } else {
        bail!("course has no entries");
    };

    if entries.is_empty() {
        bail!("course has no entries");
    }

    let title = if name.trim().is_empty() { "No Course Title".to_string() } else { name };
    let constraints =
        CourseConstraints::from_beatoraja_names(constraint.iter().map(String::as_str));
    let kind = CourseDefinition::derive_kind_from_constraints(&constraints);
    let trophies = trophy
        .into_iter()
        .map(|trophy| CourseTrophy {
            name: trophy.name,
            max_miss_rate: trophy.missrate,
            min_score_rate: trophy.scorerate,
        })
        .collect();

    let mut definition = CourseDefinition {
        key: format!("{source}#{index}"),
        title,
        kind,
        entries,
        constraints,
        trophies,
        release,
    };
    normalize_course_definition(&mut definition);
    Ok(definition)
}

fn normalize_hash(hash: String, expected_len: usize) -> Option<String> {
    let trimmed = hash.trim().to_ascii_lowercase();
    (trimmed.len() == expected_len && trimmed.chars().all(|c| c.is_ascii_hexdigit()))
        .then_some(trimmed)
}

#[cfg(test)]
mod tests {
    use bmz_core::course::{
        CourseClassConstraint, CourseGaugeConstraint, CourseKind, CourseSpeedConstraint,
    };

    use super::*;

    #[test]
    fn local_course_key_uses_first_available_sequence_number() {
        assert_eq!(next_local_course_key(["local-course-1", "local-course-3"]), "local-course-2");
    }

    #[test]
    fn parses_beatoraja_course_array() {
        let json = r#"[
          {
            "name": "七段",
            "constraint": ["grade_mirror", "no_speed", "gauge_7k"],
            "hash": [
              {"title": "Song A", "md5": "00112233445566778899aabbccddeeff", "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
            ],
            "trophy": [{"name": "gold", "missrate": 2.5, "scorerate": 88.0}]
          }
        ]"#;

        let courses = parse_beatoraja_course_json("course/default.json", json).unwrap();

        assert_eq!(courses.len(), 1);
        assert_eq!(courses[0].key, "course/default.json#0");
        assert_eq!(courses[0].title, "七段");
        assert_eq!(courses[0].kind, CourseKind::Dan);
        assert_eq!(courses[0].constraints.class, CourseClassConstraint::GradeMirrorAllowed);
        assert_eq!(courses[0].constraints.speed, CourseSpeedConstraint::NoSpeed);
        assert_eq!(courses[0].constraints.gauge, CourseGaugeConstraint::Keys7);
        assert_eq!(courses[0].entries[0].title_hint, "Song A");
        assert_eq!(courses[0].trophies[0].name, "gold");
    }

    #[test]
    fn parses_beatoraja_local_course_constraints() {
        let json = r#"[{
          "name": "Local Dan",
          "constraint": ["MIRROR", "NO_SPEED", "GAUGE_24KEYS", "HCN"],
          "hash": [{"title":"Song", "md5":"00112233445566778899aabbccddeeff"}],
          "trophy": [],
          "release": false
        }]"#;

        let courses = parse_beatoraja_course_json("course/local.json", json).unwrap();

        assert_eq!(courses[0].kind, CourseKind::Dan);
        assert_eq!(courses[0].constraints.class, CourseClassConstraint::GradeMirrorAllowed);
        assert_eq!(courses[0].constraints.gauge, CourseGaugeConstraint::Keys24);
        assert_eq!(
            courses[0].constraints.source_constraints,
            ["grade_mirror", "no_speed", "gauge_24k", "hcn"]
        );
        assert!(!courses[0].release);
    }

    #[test]
    fn serializes_beatoraja_local_course_constraint_names() {
        let mut definition = parse_beatoraja_course_json(
            "course/local.json",
            r#"[{"name":"Local","constraint":["RANDOM","NO_GREAT","GAUGE_24KEYS","CN"],"hash":[{"title":"Song","md5":"00112233445566778899aabbccddeeff"}],"trophy":[],"release":false}]"#,
        )
        .unwrap()
        .remove(0);
        normalize_course_definition(&mut definition);

        let json = serialize_beatoraja_course_json(&[definition]).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(
            value[0]["constraint"],
            serde_json::json!(["RANDOM", "NO_GREAT", "GAUGE_24KEYS", "CN"])
        );
        assert_eq!(value[0]["trophy"], serde_json::json!([]));
        assert_eq!(value[0]["release"], false);
    }

    #[test]
    fn parses_single_course_and_normalizes_defaults() {
        let json = r#"{"hash":[{"sha256":"BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"}]}"#;

        let courses = parse_beatoraja_course_json("single.json", json).unwrap();

        assert_eq!(courses[0].title, "No Course Title");
        assert_eq!(courses[0].kind, CourseKind::Course);
        assert!(courses[0].release);
        assert_eq!(courses[0].entries[0].title_hint, "course 1");
        assert_eq!(
            courses[0].entries[0].sha256.as_deref(),
            Some("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
    }

    // Stella/table-embedded format: md5 is a flat array of hex strings.
    #[test]
    fn parses_course_with_flat_md5_array() {
        let json = r#"[
          {
            "name": "Stella Skill Simulator 4th st0",
            "constraint": ["grade_mirror", "gauge_lr2", "ln"],
            "trophy": [
              {"name": "silvermedal", "missrate": 5.0, "scorerate": 70.0},
              {"name": "goldmedal",   "missrate": 2.5, "scorerate": 85.0}
            ],
            "md5": [
              "349bc491ec40d5595412637d8a4c8d2e",
              "baee0a1921fc5041b44d7d87c7b5548d",
              "72b1ce4b2051bd2a396dfa11a2d785ee",
              "3a1661a3eaafa13f976e1010d5b87ca0"
            ]
          }
        ]"#;

        let courses =
            parse_beatoraja_course_json("table:https://stellabms.xyz/st/header.json", json)
                .unwrap();

        assert_eq!(courses.len(), 1);
        assert_eq!(courses[0].title, "Stella Skill Simulator 4th st0");
        assert_eq!(courses[0].kind, CourseKind::Dan); // grade_mirror → Dan
        assert_eq!(courses[0].entries.len(), 4);
        assert_eq!(courses[0].entries[0].md5.as_deref(), Some("349bc491ec40d5595412637d8a4c8d2e"));
        assert!(courses[0].entries[0].sha256.is_none());
        assert_eq!(courses[0].trophies[0].name, "silvermedal");
        assert_eq!(courses[0].trophies[1].name, "goldmedal");
    }

    // Stella header format: course field is wrapped in an extra outer array [[...]].
    #[test]
    fn parse_courses_from_header_flattens_nested_array() {
        use crate::difficulty_table::parse_courses_from_header_for_test;
        let value = serde_json::json!([[
            {
                "name": "st0",
                "constraint": ["grade_mirror", "gauge_lr2", "ln"],
                "trophy": [],
                "md5": ["349bc491ec40d5595412637d8a4c8d2e"]
            },
            {
                "name": "st1",
                "constraint": ["grade_mirror", "gauge_lr2", "ln"],
                "trophy": [],
                "md5": ["baee0a1921fc5041b44d7d87c7b5548d"]
            }
        ]]);
        let courses = parse_courses_from_header_for_test(
            "https://stellabms.xyz/st/header.json",
            &Some(value),
        );
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].title, "st0");
        assert_eq!(courses[1].title, "st1");
    }
}
