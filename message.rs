use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract::AstarSnsContract;

impl AstarSnsContract {
    // メッセージ送信
    pub fn send_message_fn(
        &mut self,
        message: String,
        message_list_id: u128,
        sender_id: AccountId,
        createinsertd_time: String,
    ) {
        // メッセージリストの取得
        let mut message_list: Vec<Message> = self
            .message_list_map
            .get(&message_list_id)
            .unwrap_or(Vec::default());

        // メッセージの内容をメッセージリストに追加
        message_list.push(Message {
            message,
            sender_id,
            created_time,
        });

        // コントラクトを上書き
        self.message_list_map
            .insert(&message_list_id, &message_list);
    }

    // メッセージリスト取得
    pub fn get_message_list_fn(&self, message_list_id: u128, num: usize) -> Vec<Message> {
        // メッセージリストに紐ついたメッセージリストを取得
        let self_message_list: Vec<Message> = self.message_list_map.get(&message_list_id).unwrap();

        // 返り値用のメッセージリスト
        let mut message_list: Vec<Message> = Vec::new();

        // どのくらいの量の投稿を取得するか
        let amount_index: usize = 5;

        // メッセージリストの長さを指定
        let list_length: usize = self_message_list.len();

        // 投稿取得
        if list_length < amount_index + 1 {
            for m in 0..(list_length) {
                // 返り値用メッセージリストに追加
                message_list.push(self_message_list[m].clone());
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                message_list.push(self_message_list[n].clone());
            }
        }
        message_list
    }

    // メッセージリストの最後のメッセージを取得
    pub fn get_last_message_fn(&self, message_list_id: u128) -> Message {
        let last_message: Message = self
            .message_list_map
            .get(&message_list_id)
            .unwrap()
            .last()
            .unwrap()
            .clone();
        last_message
    }
}
