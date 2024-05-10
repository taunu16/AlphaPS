use crate::safe_unwrap_result;

use super::*;

pub async fn on_get_gacha_info_cs_req(
    session: &PlayerSession,
    _body: &GetGachaInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_GACHA_INFO_SC_RSP,
            GetGachaInfoScRsp {
                bgigpbnbcme: vec![
                    GachaInfo {
                        begin_time: 0,
                        end_time: 1994992000,
                        gacha_id: 2011,
                        jmdimikbkpp: 2013,
                        hjefpibalip: vec![1105, 1106, 1109],
                        ibomhpajoji: vec![1304],
                        mnnlngjlecd: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned(),
                        fcleoobmbfn: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned(),
                        eegijjhombi: std::option::Option::None,
                        ..Default::default()
                    }
                ],
                retcode: 0,
                ..Default::default()
            },
        )
        .await
}

pub async fn on_do_gacha_cs_req(
    session: &PlayerSession,
    body: &DoGachaCsReq,
) -> Result<()> {
    
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
            eginhhfhbbh: Option::None,
            fhfenbcnkei: Option::None,
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
    session
        .send(
            CMD_GET_SHOP_LIST_SC_RSP,
            GetShopListScRsp {
                shop_type: body.edhoecbcbjo,
                shop_list: vec![
                    Shop {
                        begin_time: 0,
                        end_time: 1994992000,
                        city_exp: 2137,
                        city_taken_level_reward: 0,
                        city_level: 6,
                        shop_id: 101,
                        goods_list: vec![
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 101001,
                                item_id: 101
                            },
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 101002,
                                item_id: 102
                            }
                        ]
                    },
                    Shop {
                        begin_time: 0,
                        end_time: 1994992000,
                        city_exp: 2137,
                        city_taken_level_reward: 0,
                        city_level: 6,
                        shop_id: 102,
                        goods_list: vec![
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 102001,
                                item_id: 101
                            },
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 102002,
                                item_id: 102
                            }
                        ]
                    },
                    Shop {
                        begin_time: 0,
                        end_time: 1994992000,
                        city_exp: 2137,
                        city_taken_level_reward: 0,
                        city_level: 6,
                        shop_id: 202,
                        goods_list: vec![
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 202001,
                                item_id: 101
                            },
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 202002,
                                item_id: 102
                            }
                        ]
                    },
                    Shop {
                        begin_time: 0,
                        end_time: 1994992000,
                        city_exp: 2137,
                        city_taken_level_reward: 0,
                        city_level: 6,
                        shop_id: 203,
                        goods_list: vec![
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 202001,
                                item_id: 101
                            },
                            Goods {
                                begin_time: 0,
                                end_time: 1994992000,
                                buy_times: 0,
                                goods_id: 203002,
                                item_id: 102
                            }
                        ]
                    }
                ],
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
    println!("BuyGoods {:?}", body);
    session
        .send(
            CMD_BUY_GOODS_SC_RSP, BuyGoodsScRsp { ..Default::default() }
            // BuyGoodsScRsp {
            //     retcode: 0,
            //     goods_buy_times: body.goods_num,
            //     goods_id: body.goods_id, 
            //     shop_id: body.shop_id,
            //     return_item_list: Option::Some(ItemList { item_list: vec![Item { item_id: body.item_id, num: body.goods_num, ..Default::default() }] })
            // }
        ).await
}