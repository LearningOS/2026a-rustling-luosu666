// 📖 讲解：hashmaps3
// 【题目要求】解析比赛结果文本（"队A,队B,A进球,B进球"），建立每个队的进球/失球统计表，
//             使所有测试通过。
// 【考察知识点】entry().or_insert(..) 累计统计模式：队伍第一次出现时插入零值 Team，
//             之后通过 or_insert 返回的 &mut Team 累加；
//             注意一个队的“进球”是对手的“失球”，两条 entry 调用分别更新两个队。
// 【对应教材】Rust Book §8.3（Hash Map：基于旧值更新——书中的单词计数示例就是同一模式）
// 【解法思路】对每一行：team_1 进 team_1_score 球、失 team_2_score 球；team_2 正好相反。
//             scores.entry(name).or_insert(Team { goals_scored: 0, goals_conceded: 0 })
//             返回 &mut Team，直接对字段 += 即可。
//             验证 England：对 France 进 4 失 2，对 Germany 进 1 失 2 → 进 5 失 4 ✓。

// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        // 💡 team_1：第一次出现则先插入全零 Team；or_insert 返回 &mut Team，可直接累加字段
        let team_1 = scores
            .entry(team_1_name)
            .or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_1.goals_scored += team_1_score;   // 💡 team_1 的进球
        team_1.goals_conceded += team_2_score; // 💡 team_1 的失球 = team_2 的进球

        // 💡 team_2：同样的模式，进球/失球正好与 team_1 相反
        let team_2 = scores
            .entry(team_2_name)
            .or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
