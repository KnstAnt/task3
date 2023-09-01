use argh::FromArgs;
use std::sync::mpsc::*;

#[derive(FromArgs)]
/// The application iterates over integers starting from 1,
/// calculates the sha256 hash for each of the numbers, and
/// displays the hash and the original number to the console
/// if the hash digest (character representation of the hash)
/// ends in N-characters of zero. The F parameter determines
/// how many hash values the command should find.
/// Usage example: hash_finder -N 5 -F 3
struct Args {
    /// quantity of nulls at the end of hash
    #[argh(option, short = 'N')]
    nulls: u32,

    /// quantity of hashes to find
    #[argh(option, short = 'F')]
    hashes: u32,
}

fn main() {
    let args: Args = argh::from_env();

    let mut current_number = 1;

    let mut complete_tasks = 0;
    let max_complete_tasks: usize = args.hashes as usize;

    let nulls = args.nulls as usize;

    rayon::scope(|scope| {
        let (tx, rx) = channel();

        while complete_tasks < max_complete_tasks {
            if rayon::current_num_threads() < rayon::max_num_threads() {
                let tx = tx.clone();
                scope.spawn(move |_| {
                    process_hash(current_number, nulls, tx);
                });

                current_number += 1;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            if let Ok((number, hash)) = rx.try_recv() {
                println!("{number}, {hash}");
                complete_tasks += 1;
            }
        }
    });
}

/// Computing and checking hash of number for nulls at the end.
/// In: number - target to hashing, 
/// nulls - quantity of nulls at the end of hash, 
/// tx - sends number and hash in successful case.
fn process_hash(number: usize, nulls: usize, tx: Sender<(usize, String)>) {
    let hash = sha256::digest(number.to_string());

    if let Some(index) = hash.chars().rev().position(|i| i != '0') {
        if index >= nulls {
            _ = tx.send((number, hash));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculate_hash_value_1() {
        let (tx, rx) = channel();

        let number = 4163;
        process_hash(number, 3, tx.clone());

        if let Ok(result) = rx.try_recv() {
            assert_eq!(
                (result.0, result.1.as_str()),
                (
                    number,
                    "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"
                )
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn test_calculate_hash_value_2() {
        let (tx, rx) = channel();

        let number = 828028;
        process_hash(number, 5, tx.clone());

        if let Ok(result) = rx.try_recv() {
            assert_eq!(
                (result.0, result.1.as_str()),
                (
                    number,
                    "d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000"
                )
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn test_calculate_hash_quantity_1() {
        let (tx, rx) = channel();

        let sucessful_nums = vec![4163, 11848, 12843, 13467, 20215, 28892];
        let sucessful_qnt = sucessful_nums.len();

        let unsucessful_nums = vec![1, 2, 3, 4, 5];

        sucessful_nums
            .into_iter()
            .chain(unsucessful_nums.into_iter())
            .for_each(|v| process_hash(v, 3, tx.clone()));

        let mut result_qnt = 0;

        while let Ok(_) = rx.try_recv() {
            result_qnt += 1;
        }

        assert_eq!(result_qnt, sucessful_qnt);
    }

    #[test]
    fn test_calculate_hash_quantity_2() {
        let (tx, rx) = channel();

        let sucessful_nums = vec![828028, 2513638, 3063274];
        let sucessful_qnt = sucessful_nums.len();

        let unsucessful_nums = vec![1, 2, 3, 4, 5];

        sucessful_nums
            .into_iter()
            .chain(unsucessful_nums.into_iter())
            .for_each(|v| process_hash(v, 5, tx.clone()));

        let mut result_qnt = 0;

        while let Ok(_) = rx.try_recv() {
            result_qnt += 1;
        }

        assert_eq!(result_qnt, sucessful_qnt);
    }
}
