use clap::Parser;
use clap::ValueEnum;

const MAX_REPS_TO_PRINT: u32 = 15;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum TrainingType {
    #[value(alias = "benchpress")]
    Bp,
    #[value(alias = "squat")]
    Sq,
}

impl TrainingType {
    fn coefficient(self) -> f64 {
        match self {
            Self::Bp => 40.0,
            Self::Sq => 33.3,
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about = "Estimate training weights by reps")]
struct Cli {
    #[arg(short = 't', long = "training", value_enum)]
    training_type: TrainingType,
    weight: f64,
    reps: u32,
}

fn main() {
    let cli = Cli::parse();
    let estimated_max = estimate_max(cli.weight, cli.reps, cli.training_type);

    for target_reps in 1..=MAX_REPS_TO_PRINT {
        let training_weight = weight_for_reps(estimated_max, target_reps, cli.training_type);

        println!("{target_reps:>2} {:>8}", format_weight(training_weight));
    }
}

fn estimate_max(weight: f64, reps: u32, training_type: TrainingType) -> f64 {
    weight + weight * ((reps - 1) as f64) / training_type.coefficient()
}

fn weight_for_reps(estimated_max: f64, reps: u32, training_type: TrainingType) -> f64 {
    estimated_max / (1.0 + ((reps - 1) as f64) / training_type.coefficient())
}

fn format_weight(weight: f64) -> String {
    format!("{weight:.2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchpress_estimation_round_trip_is_stable() {
        let estimated_max = estimate_max(100.0, 5, TrainingType::Bp);
        let recalculated = weight_for_reps(estimated_max, 5, TrainingType::Bp);

        assert!((recalculated - 100.0).abs() < 1e-9);
    }

    #[test]
    fn squat_estimation_round_trip_is_stable() {
        let estimated_max = estimate_max(140.0, 3, TrainingType::Sq);
        let recalculated = weight_for_reps(estimated_max, 3, TrainingType::Sq);

        assert!((recalculated - 140.0).abs() < 1e-9);
    }

    #[test]
    fn one_rep_output_uses_estimated_max() {
        let estimated_max = estimate_max(100.0, 5, TrainingType::Bp);

        assert!((estimated_max - 110.0).abs() < 1e-9);
    }

    #[test]
    fn one_rep_input_stays_the_same_weight() {
        let estimated_max = estimate_max(100.0, 1, TrainingType::Bp);
        let recalculated = weight_for_reps(estimated_max, 1, TrainingType::Bp);

        assert!((estimated_max - 100.0).abs() < 1e-9);
        assert!((recalculated - 100.0).abs() < 1e-9);
    }
}
