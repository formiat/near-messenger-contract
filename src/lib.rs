use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const MAX_MESSAGE_SIZE_BYTES: usize = 1024;

type Message = Vec<u8>;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct BlockchainMessenger {
    messages: Vec<Message>,
}

#[near_bindgen]
impl BlockchainMessenger {
    pub fn add(&mut self, message: Message) {
        if message.len() > MAX_MESSAGE_SIZE_BYTES {
            env::panic_str("Message exceeds the 1KB size limit");
        } else {
            self.messages.push(message);
        }
    }

    pub fn get(&self, index: usize) -> Message {
        if index >= self.messages.len() {
            env::panic_str("Message index out of bounds");
        } else {
            self.messages[index].clone()
        }
    }

    pub fn get_multiple(&self, start_index: usize, num_msgs: usize) -> Vec<Message> {
        if start_index >= self.messages.len() {
            env::panic_str("Start index out of bounds");
        } else {
            let end_index = std::cmp::min(start_index + num_msgs, self.messages.len());

            self.messages[start_index..end_index].to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_succ_1() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![1, 1, 1];

        contract.add(value1.clone());

        assert_eq!(value1, contract.get(0));
    }

    #[test]
    fn add_and_get_succ_2() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![1, 1, 1];
        let value2 = vec![2, 2, 2];

        contract.add(value1.clone());
        contract.add(value2.clone());

        assert_eq!(value1, contract.get(0));
        assert_eq!(value2, contract.get(1));
    }

    #[test]
    fn add_and_get_multiple_1() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![1, 1, 1];
        let value2 = vec![2, 2, 2];

        contract.add(value1.clone());
        contract.add(value2.clone());

        let values = contract.get_multiple(0, 2);

        assert_eq!(value1, values[0]);
        assert_eq!(value2, values[1]);
    }

    #[test]
    fn add_and_get_multiple_2() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![1, 1, 1];
        let value2 = vec![2, 2, 2];
        let value3 = vec![3, 3, 3];

        contract.add(value1.clone());
        contract.add(value2.clone());
        contract.add(value3.clone());

        let values = contract.get_multiple(1, 2);

        assert_eq!(value2, values[0]);
        assert_eq!(value3, values[1]);
    }

    #[test]
    #[should_panic]
    fn index_oob_1() {
        let contract = BlockchainMessenger::default();

        let _no_value = contract.get(0);
    }

    #[test]
    #[should_panic]
    fn index_oob_2() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![1, 1, 1];
        let value2 = vec![2, 2, 2];

        contract.add(value1.clone());
        contract.add(value2.clone());

        let _no_value = contract.get(2);
    }

    #[test]
    #[should_panic]
    fn start_index_oob_1() {
        let contract = BlockchainMessenger::default();

        let _no_value = contract.get_multiple(0, 1);
    }

    #[test]
    #[should_panic]
    fn start_index_oob_2() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![1, 1, 1];
        let value2 = vec![2, 2, 2];

        contract.add(value1.clone());
        contract.add(value2.clone());

        let _no_value = contract.get_multiple(2, 1);
    }

    #[test]
    #[should_panic]
    fn too_big_msg() {
        let mut contract = BlockchainMessenger::default();
        let value1 = vec![0; MAX_MESSAGE_SIZE_BYTES + 1];

        contract.add(value1.clone());
    }
}
