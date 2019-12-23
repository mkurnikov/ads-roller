use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

use crate::models::Ad;

fn get_standard_probability(total: usize) -> f32 {
    1.0 / total as f32
}

fn compute_ad_probability(ad: &Ad, total_num_ads: usize, is_last_ad: bool) -> f32 {
    let standard_ad_probability = get_standard_probability(total_num_ads);
    if is_last_ad {
        return standard_ad_probability * 0.01;
    }
    if ad.categories.is_empty() {
        return standard_ad_probability;
    }
    standard_ad_probability * (1.0 - 0.1 * ad.categories.len() as f32)
}

fn get_distribution(ads: &[Ad], total_ads: usize, last_ad_id: Option<i32>) -> WeightedIndex<f32> {
    let probs: Vec<f32> = ads
        .iter()
        .map(|ad| {
            let is_last_ad = last_ad_id.is_some() && last_ad_id.unwrap() == ad.id;
            compute_ad_probability(ad, total_ads, is_last_ad)
        })
        .collect();

    WeightedIndex::new(probs).unwrap()
}

pub fn get_sampled_ad(ads: &[Ad], total_ads: usize, last_ad_id: Option<i32>) -> &Ad {
    let dist = get_distribution(&ads, total_ads, last_ad_id);
    let mut rng = thread_rng();

    let ad_index = dist.sample(&mut rng);
    ads.get(ad_index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_an_ad(categories: Vec<String>) -> Ad {
        Ad {
            id: 1,
            url: String::from("http://localhost/"),
            num_prepaid_shows: 1,
            categories,
        }
    }

    #[test]
    fn test_probability_no_categories() {
        let ad = make_an_ad(vec![]);
        assert_eq!(compute_ad_probability(&ad, 1, false), 1.0);
        assert_eq!(compute_ad_probability(&ad, 2, false), 0.5);
        assert_eq!(compute_ad_probability(&ad, 4, false), 0.25);
    }

    #[test]
    fn test_if_banner_has_more_categories_less_probability() {
        let ad1 = make_an_ad(vec![]);
        let ad2 = make_an_ad(vec!["hello".to_string()]);

        let total = 10;
        assert!(
            compute_ad_probability(&ad1, total, false) > compute_ad_probability(&ad2, total, false)
        );
    }

    #[test]
    fn test_if_last_ad_do_not_sample() {
        // without is_last_ad, prob will be lower
        let ad1 = make_an_ad(vec![
            "hello".to_string(),
            "hello2".to_string(),
            "hello3".to_string(),
        ]);
        let ad2 = make_an_ad(vec![]);

        let total = 10;
        assert!(
            compute_ad_probability(&ad1, total, false) > compute_ad_probability(&ad2, total, true)
        )
    }

    #[test]
    fn test_sample_an_ad() {
        let ads = vec![make_an_ad(vec![]), make_an_ad(vec![])];
        let ad = get_sampled_ad(&ads, 2, None);

        assert!(ads.contains(ad));
        assert!(ads.contains(ad));
        assert!(ads.contains(ad));
    }
}
