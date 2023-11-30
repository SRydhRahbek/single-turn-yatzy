//Currently used for scoring algorithms

use crate::logic::Category;
use crate::{Board, Hand};
use std::collections::HashMap;

impl Hand {
    /*pub fn evaluate(&self, board: &Board) -> (u32, Option<Category>) {
        return (1, None)
    }*/

    pub fn evaluate_and_stack(&self) -> HashMap<Category, u32> {
        let mut eval_map = HashMap::new();
        eval_map.insert(Category::Ettor, self.eval_ettor());
        eval_map.insert(Category::Tvaor, self.eval_tvaor());
        eval_map.insert(Category::Treor, self.eval_treor());
        eval_map.insert(Category::Fyror, self.eval_fyror());
        eval_map.insert(Category::Femmor, self.eval_femmor());
        eval_map.insert(Category::Sexor, self.eval_sexor());
        eval_map.insert(Category::Par, self.eval_par());
        eval_map.insert(Category::Tretal, self.eval_tretal());
        eval_map.insert(Category::Fyrtal, self.eval_fyrtal());
        eval_map.insert(Category::Tvapar, self.eval_tvapar());
        eval_map.insert(Category::LitenStraight, self.eval_liten_straight());
        eval_map.insert(Category::StorStraight, self.eval_stor_straight());
        eval_map.insert(Category::Kak, self.eval_kak());
        eval_map.insert(Category::Chans, self.0.iter().sum());
        eval_map.insert(Category::Yatzy, self.eval_yatzy());
        return eval_map;
    }

    pub fn evaluate(&self, board: &Board) -> (u32, Option<Category>) {
        let mut best_value = 0;
        let mut best_category = None;

        //Nothing beats Yatzy
        if board
            .0
            .get(&Category::Yatzy)
            .expect("Missing Yatzy")
            .is_none()
        {
            if self.eval_yatzy() == 50 {
                return (50, Some(Category::Yatzy));
            }
        }

        if board
            .0
            .get(&Category::Chans)
            .expect("Missing Chans")
            .is_none()
        {
            //TODO: rethink this
            return (self.0.iter().sum(), Some(Category::Chans));
        }
        if board.0.get(&Category::Kak).expect("Missing Kåk").is_none() {
            let val = self.eval_kak();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Kak);
            }
        }

        if board
            .0
            .get(&Category::Sexor)
            .expect("Missing Sexor")
            .is_none()
            && best_value < 30
        {
            let val = self.eval_sexor();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Sexor);
            }
        }

        if board
            .0
            .get(&Category::Femmor)
            .expect("Missing Femmor")
            .is_none()
            && best_value < 25
        {
            let val = self.eval_femmor();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Femmor);
            }
        }

        if board
            .0
            .get(&Category::Fyrtal)
            .expect("Missing Fyrtal")
            .is_none()
            && best_value < 24
        {
            let val = self.eval_fyrtal();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Fyrtal);
            }
        }

        if board
            .0
            .get(&Category::Tvapar)
            .expect("Missing Tvåpar")
            .is_none()
            && best_value < 22
        {
            let val = self.eval_tvapar();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Tvapar);
            }
        }

        if board
            .0
            .get(&Category::StorStraight)
            .expect("Missing Stor Straight")
            .is_none()
            && best_value < 20
        {
            let val = self.eval_stor_straight();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::StorStraight);
            }
        }

        if board
            .0
            .get(&Category::Fyror)
            .expect("Missing Fyror")
            .is_none()
            && best_value < 20
        {
            let val = self.eval_fyror();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Fyror);
            }
        }

        if board
            .0
            .get(&Category::Tretal)
            .expect("Missing Tretal")
            .is_none()
            && best_value < 16
        {
            let val = self.eval_tretal();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Tretal);
            }
        }

        if board
            .0
            .get(&Category::LitenStraight)
            .expect("Missing Liten Straight")
            .is_none()
            && best_value < 15
        {
            let val = self.eval_liten_straight();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::LitenStraight);
            }
        }

        if board
            .0
            .get(&Category::Treor)
            .expect("Missing Treor")
            .is_none()
            && best_value < 15
        {
            let val = self.eval_treor();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Treor);
            }
        }

        if board.0.get(&Category::Par).expect("Missing Par").is_none() && best_value < 12 {
            let val = self.eval_par();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Par);
            }
        }

        if board
            .0
            .get(&Category::Tvaor)
            .expect("Missing Tvåor")
            .is_none()
            && best_value < 10
        {
            let val = self.eval_tvaor();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Tvaor);
            }
        }

        if board
            .0
            .get(&Category::Ettor)
            .expect("Missing Ettor")
            .is_none()
            && best_value < 5
        {
            let val = self.eval_ettor();
            if val > best_value {
                best_value = val;
                best_category = Some(Category::Ettor);
            }
        }

        return (best_value, best_category);
    }

    fn eval_ettor(&self) -> u32 {
        self.count_instances(1)
    }
    fn eval_tvaor(&self) -> u32 {
        self.count_instances(2) * 2
    }
    fn eval_treor(&self) -> u32 {
        self.count_instances(3) * 3
    }
    fn eval_fyror(&self) -> u32 {
        self.count_instances(4) * 4
    }
    fn eval_femmor(&self) -> u32 {
        self.count_instances(5) * 5
    }
    fn eval_sexor(&self) -> u32 {
        self.count_instances(6) * 6
    }

    fn eval_par(&self) -> u32 {
        for x in (1..=6).rev() {
            if self.count_instances(x) >= 2 {
                return 2 * x;
            }
        }
        return 0;
    }

    fn eval_tretal(&self) -> u32 {
        for x in (1..=6).rev() {
            if self.count_instances(x) >= 3 {
                return 3 * x;
            }
        }
        return 0;
    }

    fn eval_fyrtal(&self) -> u32 {
        for x in (1..=6).rev() {
            if self.count_instances(x) >= 4 {
                return 4 * x;
            }
        }
        return 0;
    }

    fn eval_tvapar(&self) -> u32 {
        let par_value = self.eval_par();
        let par_dice = par_value / 2;
        if par_value == 0 {
            return 0;
        } else {
            for x in (1..par_dice).rev() {
                if self.count_instances(x) >= 2 {
                    return x * 2 + par_value;
                }
            }
        }
        return 0;
    }

    fn eval_liten_straight(&self) -> u32 {
        if self == &Hand(vec![1, 2, 3, 4, 5]) {
            return 15;
        } else {
            return 0;
        }
    }

    fn eval_stor_straight(&self) -> u32 {
        if self == &Hand(vec![2, 3, 4, 5, 6]) {
            return 20;
        } else {
            return 0;
        }
    }
    /*
       fn eval_kak1(&self) -> u32 {
           let tretal_value = self.eval_tretal();
           let tretal_num = tretal_value/3;
           if tretal_value == 0 {
               return 0;
           } else {
               for x in 1..tretal_num {
                   if self.count_instances(x) >= 2 {
                       return x*2 + tretal_value;
                   }
               }
           }
           return 0;
       }
    */
    fn eval_kak(&self) -> u32 {
        if self.eval_tvapar() == 0 || self.eval_tretal() == 0 {
            return 0;
        }
        let mut score: u32 = 0;
        for x in 1..=6 {
            score += self.count_instances(x) * x;
        }
        return score;
    }

    fn eval_yatzy(&self) -> u32 {
        for x in 1..=6 {
            if self.count_instances(x) == 5 {
                return 50;
            }
        }
        return 0;
    }
}
