#![cfg(test)]

use super::curators;
use super::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok};

#[test]
fn lead_cannot_create_channel() {
    with_default_mock_builder(|| {
        assert_err!(
            Content::create_channel(
                Origin::signed(LEAD_ORIGIN),
                ContentActor::Lead,
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ),
            Error::<Test>::ActorCannotOwnChannel
        );
    })
}

#[test]
fn curator_owned_channels() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        // Curator group doesn't exist yet
        assert_err!(
            Content::create_channel(
                Origin::signed(FIRST_CURATOR_ORIGIN),
                ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ),
            Error::<Test>::CuratorGroupIsNotActive
        );

        let group_id = curators::add_curator_to_new_group(FIRST_CURATOR_ID);
        assert_eq!(FIRST_CURATOR_GROUP_ID, group_id);

        // Curator from wrong group
        assert_err!(
            Content::create_channel(
                Origin::signed(SECOND_CURATOR_ORIGIN),
                ContentActor::Curator(FIRST_CURATOR_GROUP_ID, SECOND_CURATOR_ID),
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ),
            Error::<Test>::CuratorIsNotAMemberOfGivenCuratorGroup
        );

        // Curator in correct active group, but wrong origin
        assert_err!(
            Content::create_channel(
                Origin::signed(SECOND_CURATOR_ORIGIN),
                ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ),
            Error::<Test>::CuratorAuthFailed
        );

        let channel_id = Content::next_channel_id();

        // Curator in correct active group, with correct origin
        assert_ok!(Content::create_channel(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: None,
                collaborators: BTreeSet::<MemberId>::new(),
            }
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelCreated(
                ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
                channel_id,
                ChannelRecord {
                    owner: ChannelOwner::CuratorGroup(FIRST_CURATOR_GROUP_ID),
                    videos: vec![],
                    playlists: vec![],
                    series: vec![],
                    is_censored: false,
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                },
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ))
        );

        // Curator can update channel
        assert_ok!(Content::update_channel(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
            channel_id,
            ChannelUpdateParameters {
                assets: None,
                new_meta: None,
                reward_account: None,
                maybe_collaborators: None,
            }
        ));

        // Lead can update curator owned channels
        assert_ok!(Content::update_channel(
            Origin::signed(LEAD_ORIGIN),
            ContentActor::Lead,
            channel_id,
            ChannelUpdateParameters {
                assets: None,
                new_meta: None,
                reward_account: None,
                maybe_collaborators: None,
            }
        ));
    })
}

#[test]
fn member_owned_channels() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        // Not a member
        assert_err!(
            Content::create_channel(
                Origin::signed(UNKNOWN_ORIGIN),
                ContentActor::Member(MEMBERS_COUNT + 1),
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ),
            Error::<Test>::MemberAuthFailed
        );

        let channel_id_1 = Content::next_channel_id();

        // Member can create the channel
        assert_ok!(Content::create_channel(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: None,
                collaborators: BTreeSet::<MemberId>::new(),
            }
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelCreated(
                ContentActor::Member(FIRST_MEMBER_ID),
                channel_id_1,
                ChannelRecord {
                    owner: ChannelOwner::Member(FIRST_MEMBER_ID),
                    videos: vec![],
                    playlists: vec![],
                    series: vec![],
                    is_censored: false,
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                },
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                }
            ))
        );

        let channel_id_2 = Content::next_channel_id();

        let mut collaborators = BTreeSet::new();
        collaborators.insert(COLLABORATOR_MEMBER_ID);

        // Member can create the channel
        assert_ok!(Content::create_channel(
            Origin::signed(SECOND_MEMBER_ORIGIN),
            ContentActor::Member(SECOND_MEMBER_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: None,
                collaborators: collaborators.clone(),
            }
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelCreated(
                ContentActor::Member(SECOND_MEMBER_ID),
                channel_id_2,
                ChannelRecord {
                    owner: ChannelOwner::Member(SECOND_MEMBER_ID),
                    videos: vec![],
                    playlists: vec![],
                    series: vec![],
                    is_censored: false,
                    reward_account: None,
                    collaborators: collaborators.clone(),
                },
                ChannelCreationParameters {
                    assets: vec![],
                    meta: vec![],
                    reward_account: None,
                    collaborators: collaborators.clone(),
                }
            ))
        );

        // Update channel
        assert_ok!(Content::update_channel(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            channel_id_1,
            ChannelUpdateParameters {
                assets: None,
                new_meta: None,
                reward_account: None,
                maybe_collaborators: None,
            },
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelUpdated(
                ContentActor::Member(FIRST_MEMBER_ID),
                channel_id_1,
                ChannelRecord {
                    owner: ChannelOwner::Member(FIRST_MEMBER_ID),
                    videos: vec![],
                    playlists: vec![],
                    series: vec![],
                    is_censored: false,
                    reward_account: None,
                    collaborators: BTreeSet::<MemberId>::new(),
                },
                ChannelUpdateParameters {
                    assets: None,
                    new_meta: None,
                    reward_account: None,
                    maybe_collaborators: None,
                }
            ))
        );

        // Valid collaborator should be able to update channel assets
        let assets = Some(vec![NewAsset::Urls(vec![b"test".to_vec()])]);

        // Update channel fails because channel_id_1 has no collabs
        assert_err!(
            Content::update_channel(
                Origin::signed(COLLABORATOR_MEMBER_ORIGIN),
                ContentActor::Collaborator(COLLABORATOR_MEMBER_ID),
                channel_id_1,
                ChannelUpdateParameters {
                    assets: assets.clone(),
                    new_meta: None,
                    reward_account: None,
                    maybe_collaborators: None,
                },
            ),
            Error::<Test>::ActorNotAuthorized
        );

        // Update channel succeeds because channel_id_2 has collabs
        assert_ok!(Content::update_channel(
            Origin::signed(COLLABORATOR_MEMBER_ORIGIN),
            ContentActor::Collaborator(COLLABORATOR_MEMBER_ID),
            channel_id_2,
            ChannelUpdateParameters {
                assets: assets.clone(),
                new_meta: None,
                reward_account: None,
                maybe_collaborators: None,
            },
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelUpdated(
                ContentActor::Collaborator(COLLABORATOR_MEMBER_ID),
                channel_id_2,
                ChannelRecord {
                    owner: ChannelOwner::Member(SECOND_MEMBER_ID),
                    videos: vec![],
                    playlists: vec![],
                    series: vec![],
                    is_censored: false,
                    reward_account: None,
                    collaborators: collaborators.clone(),
                },
                ChannelUpdateParameters {
                    assets: assets,
                    new_meta: None,
                    reward_account: None,
                    maybe_collaborators: None,
                }
            ))
        );

        // Member cannot update a channel they do not own
        assert_err!(
            Content::update_channel(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                ContentActor::Member(FIRST_MEMBER_ID),
                channel_id_2,
                ChannelUpdateParameters {
                    assets: None,
                    new_meta: None,
                    reward_account: None,
                    maybe_collaborators: None,
                }
            ),
            Error::<Test>::ActorNotAuthorized
        );
    })
}

#[test]
fn channel_censoring() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        let channel_id = Content::next_channel_id();
        assert_ok!(Content::create_channel(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: None,
                collaborators: BTreeSet::<MemberId>::new(),
            }
        ));

        let group_id = curators::add_curator_to_new_group(FIRST_CURATOR_ID);

        // Curator can censor channels
        let is_censored = true;
        assert_ok!(Content::update_channel_censorship_status(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(group_id, FIRST_CURATOR_ID),
            channel_id,
            is_censored,
            vec![]
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelCensorshipStatusUpdated(
                ContentActor::Curator(group_id, FIRST_CURATOR_ID),
                channel_id,
                is_censored,
                vec![]
            ))
        );

        let channel = Content::channel_by_id(channel_id);

        assert!(channel.is_censored);

        // Curator can un-censor channels
        let is_censored = false;
        assert_ok!(Content::update_channel_censorship_status(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(group_id, FIRST_CURATOR_ID),
            channel_id,
            is_censored,
            vec![]
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ChannelCensorshipStatusUpdated(
                ContentActor::Curator(group_id, FIRST_CURATOR_ID),
                channel_id,
                is_censored,
                vec![]
            ))
        );

        let channel = Content::channel_by_id(channel_id);

        assert!(!channel.is_censored);

        // Member cannot censor channels
        let is_censored = true;
        assert_err!(
            Content::update_channel_censorship_status(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                ContentActor::Member(FIRST_MEMBER_ID),
                channel_id,
                is_censored,
                vec![]
            ),
            Error::<Test>::ActorNotAuthorized
        );

        let curator_channel_id = Content::next_channel_id();

        // create curator channel
        assert_ok!(Content::create_channel(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(group_id, FIRST_CURATOR_ID),
            ChannelCreationParameters {
                assets: vec![],
                meta: vec![],
                reward_account: None,
                collaborators: BTreeSet::<MemberId>::new(),
            }
        ));

        // Curator cannot censor curator group channels
        assert_err!(
            Content::update_channel_censorship_status(
                Origin::signed(FIRST_CURATOR_ORIGIN),
                ContentActor::Curator(group_id, FIRST_CURATOR_ID),
                curator_channel_id,
                is_censored,
                vec![]
            ),
            Error::<Test>::CannotCensoreCuratorGroupOwnedChannels
        );

        // Lead can still censor curator group channels
        assert_ok!(Content::update_channel_censorship_status(
            Origin::signed(LEAD_ORIGIN),
            ContentActor::Lead,
            curator_channel_id,
            is_censored,
            vec![]
        ));
    })
}
