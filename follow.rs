use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract::AstarSnsContract;

impl AstarSnsContract {
    // follow func
    pub fn follow_fn(&mut self, following_account_id: AccountId, followed_account_id: AccountId) {
        // ユーザがフォローしているユーザのプロフィール情報を取得
        let mut following_user_profile: Profile =
            self.profile_map.get(&following_account_id).unwrap();

        // ユーザがフォローされているユーザのプロフィール情報を取得
        let mut followed_user_profile: Profile =
            self.profile_map.get(&followed_account_id).unwrap();

        // まだフォローしていない場合、自分のフォローするユーザのアドレスを自分のフォローリストに追加
        if !following_user_profile
            .following_list
            .contains(&followed_account_id)
        {
            following_user_profile
                .following_list
                .push(followed_account_id);
        }

        // フォローされる側のフォロワーリストに自分が含まれていなければ追加
        if !followed_user_profile
            .follower_list
            .contains(&following_account_id)
        {
            followed_user_profile
                .follower_list
                .push(following_account_id);
        }

        // 自分のメッセージリストのIDのリストの長さを取得
        let length: usize = following_user_profile.message_list_id_list.len();

        // 自分かフォローするアカウントのどちらかがメッセージリストを持っていないか確認
        if followed_user_profile.message_list_id_list.len() == 0
            && following_user_profile.message_list_id_list.len() == 0
        {
            // 両方のメッセージリストIDをそれぞれに追加
            followed_user_profile
                .message_list_id_list
                .push(self.message_list_map_counter);
            following_user_profile
                .message_list_id_list
                .push(self.message_list_map_counter);
            following_user_profile.friend_list.push(followed_account_id);
            followed_user_profile.friend_list.push(following_account_id);

            // メッセージリストのIDを+1
            self.message_list_map_counter = &self.message_list_map_counter + 1;
        }

        for n in 0..length {
            // 自分の持っているメッセージリストのIDをフォローする相手が持っていない
            // すでに二人用のメッセージリストが作成されていないかを確認
            let is_contained = followed_user_profile
                .message_list_id_list
                .contains(&following_user_profile.message_list_id_list[n]);

            // もし含まれていなければメッセージリストのIDを追加
            if !is_contained {
                followed_user_profile
                    .message_list_id_list
                    .push(self.message_list_map_counter);
                following_user_profile
                    .message_list_id_list
                    .push(self.message_list_map_counter);
                following_user_profile.friend_list.push(followed_account_id);
                followed_user_profile.friend_list.push(following_account_id);
                self.message_list_map_counter = &self.message_list_map_counter + 1;
            }
        }

        // それぞれのプロフィール情報を上書き
        self.profile_map
            .insert(&following_account_id, &following_user_profile);
        self.profile_map
            .insert(&followed_account_id, &followed_user_profile);
    }

    // フォローリストを取得
    pub fn get_following_list_fn(&self, account_id: AccountId) -> Vec<AccountId> {
        let following_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().following_list;
        following_list
    }

    // 指定したユーザがフォローしているアカウントのフォロワーリストを取得
    pub fn get_follower_list_fn(&self, account_id: AccountId) -> Vec<AccountId> {
        let follower_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().follower_list;
        follower_list
    }
}
