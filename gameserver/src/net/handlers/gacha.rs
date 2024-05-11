use std::collections::HashMap;

use crate::{excel::{types::{GachaType, ItemType}, EXCEL}, safe_unwrap_result};

use super::*;

pub async fn on_get_gacha_info_cs_req(
    session: &PlayerSession,
    _body: &GetGachaInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_GACHA_INFO_SC_RSP,
            GetGachaInfoScRsp {
                bgigpbnbcme: EXCEL.gacha_basic_info.iter()
                    .filter(|banner| !["UI/Drawcard/GachaPanelTemplate/AvatarGacha_RuanMei.prefab", "UI/Drawcard/GachaPanel/AvatarGacha_2015.prefab", "UI/Drawcard/GachaPanel/LightConeGacha_3015.prefab", "UI/Drawcard/GachaPanelTemplate/LightConeGacha_23019.prefab"].contains(&banner.prefab_path.as_str()))
                    .map(|banner| GachaInfo {
                    begin_time: 0,
                    end_time: if [GachaType::AvatarUp, GachaType::WeaponUp].contains(&banner.gacha_type) { i64::MAX } else { 0 },
                    gacha_id: banner.gacha_id,
                    ibomhpajoji: vec![1105, 1106, 1109],
                    hjefpibalip: vec![1310],
                    mnnlngjlecd: "".to_owned(),
                    fcleoobmbfn: "".to_owned(),
                    eegijjhombi: std::option::Option::None,
                    ..Default::default()
                }).collect(),
                /*vec![
                    GachaInfo {
                        begin_time: 0,
                        end_time: 1994992000,
                        gacha_id: 2011,
                        ibomhpajoji: vec![1105, 1106, 1109],
                        hjefpibalip: vec![1310],
                        mnnlngjlecd: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned(),
                        fcleoobmbfn: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned(),
                        eegijjhombi: std::option::Option::None,
                        ..Default::default()
                    }
                ],*/
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}

pub async fn on_do_gacha_cs_req(
    session: &PlayerSession,
    body: &DoGachaCsReq,
) -> Result<()> {println!("{:?}", body);
    
    safe_unwrap_result!(session.player_info_mut().inventory.take_item(session, if body.gacha_id < 2000 { 101 } else { 102 }, body.gacha_num).await);

    let mut items = vec![];

    for _ in 0..body.gacha_num {
        items.push(GachaItem {
            is_new: true,
            gacha_item: Some(Item {
                item_id: 1310,
                num: 1,
                ..Default::default()
            }),
            eginhhfhbbh: Some(ItemList { item_list: vec![] }),
            fhfenbcnkei: Some(ItemList { item_list: vec![] }),
            // ibeeoiikkdf: Some(ItemList{ item_list: vec![/*Item {item_id: 101, num: 1, ..Default::default()}*/] }),
            // oopkehibdfd: Some(ItemList{ item_list: vec![/*Item {item_id: 102, num: 1, ..Default::default()}*/] }),
            ..Default::default()
        });
    }

    session
        .send(
            CMD_DO_GACHA_SC_RSP,
            DoGachaScRsp {
                gacha_num: body.gacha_num,
                gacha_id: body.gacha_id,
                gcejognjecl: items,
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_shop_list_cs_req(
    session: &PlayerSession,
    body: &GetShopListCsReq,
) -> Result<()> {
    let mut shop_config: HashMap<u32, Vec<crate::excel::types::ShopGoodsConfigElement>> = HashMap::new();

    for shop in &EXCEL.shop_goods_config {
        if let Some(shops) = shop_config.get_mut(&shop.shop_id) {
            shops.push(shop.clone());
        } else {
            shop_config.insert(shop.shop_id, vec![shop.clone()]);
        }
    }

    session
        .send(
            CMD_GET_SHOP_LIST_SC_RSP,
            GetShopListScRsp {
                shop_type: body.edhoecbcbjo,
                shop_list: shop_config.into_iter().map(|(shop_id, goods)| { Shop {
                    shop_id,
                    begin_time: 0,
                    end_time: i64::MAX,
                    city_exp: 2137,
                    city_level: 6,
                    city_taken_level_reward: 0,
                    goods_list: goods.iter().map(|good| Goods {
                        begin_time: 0,
                        end_time: i64::MAX,
                        item_id: good.item_id,
                        buy_times: 0,
                        goods_id: good.goods_id
                    }).collect()
                } }).collect(),
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}

pub async fn on_buy_goods_cs_req(
    session: &PlayerSession,
    body: &BuyGoodsCsReq,
) -> Result<()> {
    let player_info = &mut session.player_info_mut();

    let mut item = if let Some(good) = EXCEL.shop_goods_config.iter().find(|good| good.goods_id == body.goods_id) {
        Item {
            item_id: good.item_id,
            level: good.level,
            num: body.goods_num,
            rank: good.rank,
            ..Default::default()
        }
    } else {
        Item { 
            item_id: body.item_id, 
            num: body.goods_num, 
            ..Default::default() 
        }
    };

    if let Some(item_cfg) = EXCEL.item.item_config.iter().find(|itm| item.item_id == itm.id) {
        if [ItemType::Relic, ItemType::Equipment].contains(&item_cfg.item_main_type) { //those require custom id; relics and lightcones
            item.unique_id = player_info.inventory.find_free_id();
        }

        safe_unwrap_result!(match item_cfg.item_main_type {
            ItemType::Relic => player_info.inventory.add_relic(&session, item.item_id, item.level, 1, vec![]).await,
            ItemType::Equipment => player_info.inventory.add_lightcone(&session, item.item_id, item.level, item.rank, item.promotion).await,
            _ => player_info.inventory.add_item(&session, item.item_id, item.num).await
        })
    } else {
        safe_unwrap_result!(player_info.inventory.add_item(&session, item.item_id, item.num).await);
    }

    session
        .send(
            CMD_BUY_GOODS_SC_RSP, 
            BuyGoodsScRsp {
                retcode: 0,
                goods_buy_times: body.goods_num,
                goods_id: body.goods_id, 
                shop_id: body.shop_id,
                return_item_list: Option::Some(ItemList { item_list: vec![ item ] })
            }
        ).await
}