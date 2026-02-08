# NUJ Monitor - NLP Threshold Self-Tuning Configuration
# Using Nickel configuration language for type-safe, composable configs

{
  # Supervised Learning Configuration
  supervised = {
    # Training data collection
    training = {
      enabled = true,
      feedback_collection = {
        # Collect human feedback on AI decisions
        collect_on_approval = true,
        collect_on_rejection = true,
        collect_on_rollback = true,
        min_samples_for_retraining = 100,
      },

      # Feature extraction
      features = [
        "change_word_count",
        "change_char_count",
        "sentiment_shift",
        "section_count_delta",
        "policy_keywords_frequency",
        "legal_terms_frequency",
        "journalist_impact_indicators",
      ],

      # Training schedule
      retraining = {
        frequency = "weekly",
        validation_split = 0.2,
        test_split = 0.1,
        min_accuracy_threshold = 0.75,
      },
    },

    # Model configuration
    models = {
      severity_classifier = {
        type = "random_forest",
        parameters = {
          n_estimators = 100,
          max_depth = 10,
          min_samples_split = 5,
        },
        thresholds = {
          critical = 0.85,
          high = 0.70,
          medium = 0.55,
          low = 0.40,
        },
      },

      impact_regressor = {
        type = "gradient_boosting",
        parameters = {
          learning_rate = 0.1,
          n_estimators = 150,
          max_depth = 7,
        },
      },

      notification_classifier = {
        type = "logistic_regression",
        parameters = {
          C = 1.0,
          max_iter = 1000,
        },
        threshold = 0.65,
      },
    },
  },

  # Unsupervised Learning Configuration
  unsupervised = {
    # Anomaly detection
    anomaly_detection = {
      enabled = true,
      algorithm = "isolation_forest",
      parameters = {
        contamination = 0.1,
        n_estimators = 100,
      },
      # Flag unusual changes for manual review
      auto_flag_anomalies = true,
    },

    # Clustering for pattern discovery
    clustering = {
      enabled = true,
      algorithm = "dbscan",
      parameters = {
        eps = 0.5,
        min_samples = 5,
      },
      # Discover common change patterns
      pattern_discovery = true,
      min_cluster_size = 10,
    },

    # Topic modeling
    topic_modeling = {
      enabled = true,
      algorithm = "lda",
      parameters = {
        n_topics = 10,
        alpha = 0.1,
        beta = 0.01,
      },
      # Extract emerging themes in policy changes
      track_topic_evolution = true,
    },
  },

  # Semi-Automated Self-Tuning with SLM
  self_tuning = {
    enabled = true,

    # Small Language Model for threshold optimization
    slm = {
      model = "microsoft/phi-2",  # 2.7B parameter model
      quantization = "int8",  # For efficiency
      device = "cpu",  # Can run on CPU

      # Threshold tuning strategy
      tuning = {
        method = "bayesian_optimization",
        objective = "f1_score",
        n_iterations = 50,
        exploration_weight = 0.3,
      },

      # Parameter search space
      search_space = {
        severity_thresholds = {
          critical = { min = 0.75, max = 0.95 },
          high = { min = 0.60, max = 0.80 },
          medium = { min = 0.45, max = 0.65 },
          low = { min = 0.30, max = 0.50 },
        },

        confidence_thresholds = {
          auto_notify = { min = 0.60, max = 0.90 },
          require_review = { min = 0.40, max = 0.70 },
        },

        feature_weights = {
          sentiment_weight = { min = 0.1, max = 0.5 },
          keyword_weight = { min = 0.2, max = 0.6 },
          structural_weight = { min = 0.1, max = 0.4 },
        },
      },
    },

    # Adaptive threshold adjustment
    adaptive = {
      enabled = true,

      # Monitor performance metrics
      metrics_tracking = {
        false_positive_rate = {
          target = 0.05,
          tolerance = 0.02,
        },
        false_negative_rate = {
          target = 0.01,
          tolerance = 0.005,
        },
        precision = {
          target = 0.90,
          min_acceptable = 0.85,
        },
        recall = {
          target = 0.95,
          min_acceptable = 0.90,
        },
      },

      # Adjustment strategy
      adjustment = {
        frequency = "daily",
        max_change_per_iteration = 0.05,
        convergence_threshold = 0.01,
        lookback_window_days = 30,
      },

      # Safety constraints
      safety = {
        # Never lower these thresholds
        min_critical_threshold = 0.75,
        min_notification_confidence = 0.60,

        # Always require human review for these
        always_review_critical = true,
        always_review_low_confidence = true,

        # Rollback if performance degrades
        auto_rollback_on_degradation = true,
        degradation_threshold = 0.1,
      },
    },

    # A/B testing framework
    ab_testing = {
      enabled = true,
      # Test new thresholds on subset of data
      test_fraction = 0.2,
      min_test_samples = 50,
      statistical_significance = 0.05,
    },
  },

  # Real-time monitoring
  monitoring = {
    # Performance dashboard
    dashboard = {
      update_frequency_seconds = 60,
      metrics = [
        "accuracy",
        "precision",
        "recall",
        "f1_score",
        "false_positive_rate",
        "false_negative_rate",
        "average_confidence",
        "threshold_drift",
      ],
    },

    # Alerts
    alerts = {
      performance_degradation = {
        enabled = true,
        threshold = 0.1,  # 10% degradation
        notify = ["comms@nuj.org.uk"],
      },
      threshold_drift = {
        enabled = true,
        max_drift = 0.15,  # 15% from baseline
        notify = ["tech@nuj.org.uk"],
      },
    },
  },

  # Explainability
  explainability = {
    enabled = true,
    # Generate explanations for threshold adjustments
    log_adjustments = true,
    # SHAP values for feature importance
    shap_analysis = true,
    # Human-readable summaries
    generate_summaries = true,
  },

  # Data retention for training
  data_retention = {
    training_samples = {
      retention_days = 365,
      max_samples = 100000,
    },
    model_versions = {
      retention_count = 10,
      keep_best = true,
    },
  },
}
