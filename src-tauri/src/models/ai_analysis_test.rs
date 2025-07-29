//! AIAnalysisアルゴリズムのテスト
//! レビューフィードバック適用：0-100範囲の境界値テスト

#[cfg(test)]
mod tests {
    use super::super::{AIAnalysis, UrgencyFactors};
    use chrono::{DateTime, Utc, Duration};

    #[test]
    fn test_calculate_final_score_minimum_values() {
        // 最小値のテスト (0, 0, 0, 1)
        let analysis = AIAnalysis::new(
            "test-ticket-min".to_string(),
            0.0,  // urgency
            0.0,  // complexity
            0.0,  // user_relevance
            1.0,  // project_weight (最小)
            "最小値テスト".to_string(),
            "test".to_string(),
        );

        // 期待値: (0 * 0.4 + 0 * 0.3 + 0 * 0.3) * (1 / 5) = 0 * 0.2 = 0.0
        assert_eq!(analysis.final_priority_score, 0.0);
    }

    #[test]
    fn test_calculate_final_score_maximum_values() {
        // 最大値のテスト (100, 100, 100, 10)
        let analysis = AIAnalysis::new(
            "test-ticket-max".to_string(),
            100.0,  // urgency
            100.0,  // complexity
            100.0,  // user_relevance
            10.0,   // project_weight (最大)
            "最大値テスト".to_string(),
            "test".to_string(),
        );

        // 期待値: (100 * 0.4 + 100 * 0.3 + 100 * 0.3) * (10 / 5) = 100 * 2 = 200
        // ただし100でクランプされる
        assert_eq!(analysis.final_priority_score, 100.0);
    }

    #[test]
    fn test_calculate_final_score_boundary_values() {
        // 境界値テストケース
        let test_cases = vec![
            // (urgency, complexity, user_relevance, project_weight, expected_max_score)
            (0.0, 0.0, 0.0, 1.0, 0.0),    // 全て最小
            (100.0, 100.0, 100.0, 10.0, 100.0), // 全て最大（クランプ）
            (50.0, 50.0, 50.0, 5.0, 50.0),      // 中央値
            (100.0, 0.0, 0.0, 1.0, 8.0),        // 緊急度のみ最大、重み最小
            (0.0, 100.0, 0.0, 1.0, 6.0),        // 複雑度のみ最大、重み最小
            (0.0, 0.0, 100.0, 1.0, 6.0),        // ユーザー関連度のみ最大、重み最小
            (50.0, 50.0, 50.0, 1.0, 10.0),      // 各スコア中央値、重み最小
            (50.0, 50.0, 50.0, 10.0, 100.0),    // 各スコア中央値、重み最大（クランプ）
        ];

        for (urgency, complexity, user_relevance, project_weight, expected) in test_cases {
            let analysis = AIAnalysis::new(
                format!("test-ticket-{}-{}-{}-{}", urgency, complexity, user_relevance, project_weight),
                urgency,
                complexity,
                user_relevance,
                project_weight,
                "境界値テスト".to_string(),
                "test".to_string(),
            );

            // 浮動小数点数の比較のため、小さな誤差を許容
            let difference = (analysis.final_priority_score - expected).abs();
            assert!(
                difference < 0.01,
                "スコア計算が期待値と一致しません。期待値: {}, 実際: {}, 入力: ({}, {}, {}, {})",
                expected, analysis.final_priority_score, urgency, complexity, user_relevance, project_weight
            );
        }
    }

    #[test]
    fn test_calculate_final_score_algorithm_correctness() {
        // アルゴリズムの正確性テスト
        let urgency = 80.0;
        let complexity = 60.0;
        let user_relevance = 40.0;
        let project_weight = 6.0;

        let analysis = AIAnalysis::new(
            "test-algorithm".to_string(),
            urgency,
            complexity,
            user_relevance,
            project_weight,
            "アルゴリズムテスト".to_string(),
            "test".to_string(),
        );

        // 手動計算: (80 * 0.4 + 60 * 0.3 + 40 * 0.3) * (6 / 5)
        // = (32 + 18 + 12) * 1.2
        // = 62 * 1.2 = 74.4
        let expected = 74.4;
        let difference = (analysis.final_priority_score - expected).abs();
        assert!(
            difference < 0.01,
            "アルゴリズムが正しく実装されていません。期待値: {}, 実際: {}",
            expected, analysis.final_priority_score
        );
    }

    #[test]
    fn test_calculate_final_score_negative_values() {
        // 負の値での動作テスト（クランプされて0になる）
        let analysis = AIAnalysis::new(
            "test-negative".to_string(),
            -10.0,  // 負の緊急度
            -5.0,   // 負の複雑度
            -15.0,  // 負のユーザー関連度
            1.0,    // 最小重み
            "負の値テスト".to_string(),
            "test".to_string(),
        );

        // 負の値は0にクランプされる
        assert_eq!(analysis.final_priority_score, 0.0);
    }

    #[test]
    fn test_calculate_final_score_extreme_values() {
        // 極端に大きな値での動作テスト
        let analysis = AIAnalysis::new(
            "test-extreme".to_string(),
            1000.0,  // 極端に大きな緊急度
            2000.0,  // 極端に大きな複雑度
            3000.0,  // 極端に大きなユーザー関連度
            100.0,   // 極端に大きな重み
            "極端な値テスト".to_string(),
            "test".to_string(),
        );

        // 100でクランプされる
        assert_eq!(analysis.final_priority_score, 100.0);
    }

    #[test]
    fn test_calculate_final_score_zero_project_weight() {
        // プロジェクト重みが0の場合のテスト
        let analysis = AIAnalysis::new(
            "test-zero-weight".to_string(),
            100.0,  // 最大緊急度
            100.0,  // 最大複雑度
            100.0,  // 最大ユーザー関連度
            0.0,    // 重み0
            "重み0テスト".to_string(),
            "test".to_string(),
        );

        // 重みが0の場合、最終スコアは0になる
        assert_eq!(analysis.final_priority_score, 0.0);
    }

    #[test]
    fn test_project_weight_normalization() {
        // プロジェクト重みの正規化テスト
        let test_cases = vec![
            (1.0, 0.2),   // 1 / 5 = 0.2
            (2.5, 0.5),   // 2.5 / 5 = 0.5
            (5.0, 1.0),   // 5 / 5 = 1.0
            (7.5, 1.5),   // 7.5 / 5 = 1.5
            (10.0, 2.0),  // 10 / 5 = 2.0
        ];

        for (project_weight, expected_multiplier) in test_cases {
            let analysis = AIAnalysis::new(
                format!("test-weight-{}", project_weight),
                50.0,  // 固定値
                50.0,  // 固定値
                50.0,  // 固定値
                project_weight,
                "重み正規化テスト".to_string(),
                "test".to_string(),
            );

            // 基本スコア: 50 * 0.4 + 50 * 0.3 + 50 * 0.3 = 50
            // 最終スコア: 50 * expected_multiplier
            let expected_score = 50.0 * expected_multiplier;
            let difference = (analysis.final_priority_score - expected_score).abs();
            assert!(
                difference < 0.01,
                "プロジェクト重み正規化が正しくありません。重み: {}, 期待値: {}, 実際: {}",
                project_weight, expected_score, analysis.final_priority_score
            );
        }
    }

    #[test]
    fn test_score_distribution_weights() {
        // スコア配分の重みテスト（緊急度40%, 複雑度30%, ユーザー関連度30%）
        let urgency_only = AIAnalysis::new(
            "urgency-only".to_string(),
            100.0, 0.0, 0.0, 5.0,  // 緊急度のみ
            "緊急度のみ".to_string(),
            "test".to_string(),
        );

        let complexity_only = AIAnalysis::new(
            "complexity-only".to_string(),
            0.0, 100.0, 0.0, 5.0,  // 複雑度のみ
            "複雑度のみ".to_string(),
            "test".to_string(),
        );

        let user_relevance_only = AIAnalysis::new(
            "user-relevance-only".to_string(),
            0.0, 0.0, 100.0, 5.0,  // ユーザー関連度のみ
            "ユーザー関連度のみ".to_string(),
            "test".to_string(),
        );

        // 緊急度が最も高い重みを持つことを確認
        assert!(urgency_only.final_priority_score > complexity_only.final_priority_score);
        assert!(urgency_only.final_priority_score > user_relevance_only.final_priority_score);

        // 複雑度とユーザー関連度は同じ重み
        let difference = (complexity_only.final_priority_score - user_relevance_only.final_priority_score).abs();
        assert!(difference < 0.01, "複雑度とユーザー関連度の重みが同じでありません");

        // 具体的な値の確認
        assert!((urgency_only.final_priority_score - 40.0).abs() < 0.01);  // 100 * 0.4 * 1.0
        assert!((complexity_only.final_priority_score - 30.0).abs() < 0.01);  // 100 * 0.3 * 1.0
        assert!((user_relevance_only.final_priority_score - 30.0).abs() < 0.01);  // 100 * 0.3 * 1.0
    }

    #[test]
    fn test_urgency_factors_boundary_values() {
        // UrgencyFactorsの境界値テスト
        let base_time = Utc::now();

        // 期限切れチケット（1日前）
        let overdue_factors = UrgencyFactors {
            due_date: Some(base_time - Duration::days(1)),
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let overdue_multiplier = overdue_factors.calculate_urgency_multiplier();
        assert_eq!(overdue_multiplier, 2.0);

        // 1日後の期限（確実に1日後とするため25時間後）
        let one_day_factors = UrgencyFactors {
            due_date: Some(base_time + Duration::hours(25)),
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let one_day_multiplier = one_day_factors.calculate_urgency_multiplier();
        assert_eq!(one_day_multiplier, 1.8);

        // 2-3日以内の期限
        let three_day_factors = UrgencyFactors {
            due_date: Some(base_time + Duration::days(3)),
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let three_day_multiplier = three_day_factors.calculate_urgency_multiplier();
        assert_eq!(three_day_multiplier, 1.5);

        // 1週間以内の期限
        let week_factors = UrgencyFactors {
            due_date: Some(base_time + Duration::days(7)),
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let week_multiplier = week_factors.calculate_urgency_multiplier();
        assert_eq!(week_multiplier, 1.2);

        // 期限が遠い
        let far_factors = UrgencyFactors {
            due_date: Some(base_time + Duration::days(30)),
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let far_multiplier = far_factors.calculate_urgency_multiplier();
        assert_eq!(far_multiplier, 1.0);

        // 期限なし
        let no_due_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let no_due_multiplier = no_due_factors.calculate_urgency_multiplier();
        assert_eq!(no_due_multiplier, 1.0);
    }

    #[test]
    fn test_urgency_factors_comment_activity() {
        // コメント活動による緊急度テスト
        let high_comment_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 5,  // > 3
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let high_comment_multiplier = high_comment_factors.calculate_urgency_multiplier();
        assert_eq!(high_comment_multiplier, 1.3);

        let low_comment_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 2,  // <= 3
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let low_comment_multiplier = low_comment_factors.calculate_urgency_multiplier();
        assert_eq!(low_comment_multiplier, 1.0);
    }

    #[test]
    fn test_urgency_factors_mentions() {
        // メンション数による緊急度テスト
        let high_mention_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 0,
            mentions_count: 3,  // > 1
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let high_mention_multiplier = high_mention_factors.calculate_urgency_multiplier();
        assert_eq!(high_mention_multiplier, 1.2);

        let low_mention_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 0,
            mentions_count: 1,  // <= 1
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: false,
        };
        let low_mention_multiplier = low_mention_factors.calculate_urgency_multiplier();
        assert_eq!(low_mention_multiplier, 1.0);
    }

    #[test]
    fn test_urgency_factors_assignment_and_blocking() {
        // 担当者チケットとブロッカーチケットのテスト
        let assigned_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: true,
            is_blocking_other_tickets: false,
        };
        let assigned_multiplier = assigned_factors.calculate_urgency_multiplier();
        assert_eq!(assigned_multiplier, 1.1);

        let blocking_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: false,
            is_blocking_other_tickets: true,
        };
        let blocking_multiplier = blocking_factors.calculate_urgency_multiplier();
        assert_eq!(blocking_multiplier, 1.5);

        // 両方の条件が満たされた場合
        let both_factors = UrgencyFactors {
            due_date: None,
            recent_comments: 0,
            mentions_count: 0,
            last_update_days: 0,
            is_assigned_to_user: true,
            is_blocking_other_tickets: true,
        };
        let both_multiplier = both_factors.calculate_urgency_multiplier();
        assert_eq!(both_multiplier, 1.1 * 1.5); // 1.65
    }

    #[test]
    fn test_urgency_factors_combined_maximum() {
        // 全ての緊急度要因が最大の場合
        let now = Utc::now();
        let max_factors = UrgencyFactors {
            due_date: Some(now - Duration::days(1)),  // 期限切れ: 2.0x
            recent_comments: 10,                      // 高コメント: 1.3x
            mentions_count: 5,                        // 高メンション: 1.2x
            last_update_days: 0,
            is_assigned_to_user: true,                // 担当者: 1.1x
            is_blocking_other_tickets: true,          // ブロッカー: 1.5x
        };
        let max_multiplier = max_factors.calculate_urgency_multiplier();
        let expected = 2.0 * 1.3 * 1.2 * 1.1 * 1.5; // 5.148
        assert!((max_multiplier - expected).abs() < 0.01);
    }

    #[test]
    fn test_ai_analysis_complete_workflow() {
        // AI分析の完全なワークフローテスト
        let now = Utc::now();
        let urgency_factors = UrgencyFactors {
            due_date: Some(now + Duration::days(2)),  // 2日後期限: 1.5x
            recent_comments: 5,                       // 高コメント: 1.3x
            mentions_count: 2,                        // 高メンション: 1.2x
            last_update_days: 1,
            is_assigned_to_user: true,                // 担当者: 1.1x
            is_blocking_other_tickets: false,
        };

        let urgency_multiplier = urgency_factors.calculate_urgency_multiplier();
        let expected_multiplier = 1.5 * 1.3 * 1.2 * 1.1; // 2.574

        // 緊急度に乗数を適用
        let base_urgency = 60.0;
        let adjusted_urgency = base_urgency * urgency_multiplier;

        let analysis = AIAnalysis::new(
            "workflow-test".to_string(),
            adjusted_urgency.min(100.0), // 100でクランプ
            70.0,  // complexity
            80.0,  // user_relevance
            8.0,   // project_weight
            "ワークフローテスト".to_string(),
            "integration".to_string(),
        );

        // 結果が妥当な範囲内にあることを確認
        assert!(analysis.final_priority_score >= 0.0);
        assert!(analysis.final_priority_score <= 100.0);

        // 入力値が正しく保存されていることを確認
        assert_eq!(analysis.ticket_id, "workflow-test");
        assert_eq!(analysis.complexity_score, 70.0);
        assert_eq!(analysis.user_relevance_score, 80.0);
        assert_eq!(analysis.project_weight_factor, 8.0);
        assert_eq!(analysis.category, "integration");
    }
}