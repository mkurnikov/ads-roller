use rand::distributions::WeightedIndex;

use crate::models::Ad;

pub fn get_standard_probability(total: usize) -> f32 {
    1.0 / total as f32
}

pub fn compute_ad_probability(ad: &Ad, total_num_ads: usize) -> f32 {
    let standard_ad_probability = get_standard_probability(total_num_ads);
    if ad.categories.is_empty() {
        return standard_ad_probability;
    }
    standard_ad_probability * (1.0 - 0.1 * ad.categories.len() as f32)
}

pub fn get_distribution(ads: &[Ad], total_ads: usize, last_ad: Option<Ad>) -> WeightedIndex<f32> {
    let probs: Vec<f32> = ads
        .iter()
        .map(|ad| {
            if let Some(last_ad) = &last_ad {
                if ad == last_ad {
                    return get_standard_probability(total_ads) * 0.01;
                }
            }
            compute_ad_probability(ad, total_ads)
        })
        .collect();

    WeightedIndex::new(probs).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_no_categories() {
        let ad = Ad {
            id: 0,
            url: "http://localhost".to_string(),
            num_prepaid_shows: 10,
            categories: vec![],
        };
        assert_eq!(compute_ad_probability(&ad, 1), 1.0);
        assert_eq!(compute_ad_probability(&ad, 2), 0.5);
        assert_eq!(compute_ad_probability(&ad, 4), 0.25);
    }

    #[test]
    fn test_if_banner_has_more_categories_less_probability() {
        let ad1 = Ad {
            id: 0,
            url: "http://localhost".to_string(),
            num_prepaid_shows: 10,
            categories: vec![],
        };
        let ad2 = Ad {
            id: 0,
            url: "http://localhost".to_string(),
            num_prepaid_shows: 10,
            categories: vec!["hello".to_string(), "world".to_string()],
        };

        let total = 10;
        assert!(compute_ad_probability(&ad1, total) > compute_ad_probability(&ad2, total));
    }
}
