pub const ALL_ITEMS: [u32; 3013] = [1,2,3,11,12,21,22,23,24,25,31,32,33,41,51,52,53,101,102,170,171,201,211,212,213,221,222,223,231,232,233,234,235,236,241,251,252,261,262,271,503,800,801,802,71000,71001,71002,71003,71004,71005,71006,71007,71008,71009,71010,71011,71012,71013,71014,71015,71016,71017,71018,71019,81011,81012,81013,81014,81021,81022,81023,81024,81031,81032,81033,81034,81041,81042,81043,81044,81051,81052,81053,81054,81061,81062,81063,81064,81071,81072,81073,81074,81081,81082,81083,81084,81091,81092,81093,81094,81101,81102,81103,81104,81111,81112,81113,81114,81121,81122,81123,81124,81131,81132,81133,81134,81141,81142,81143,81144,81151,81152,81153,81154,81161,81162,81163,81164,81171,81172,81173,81174,81181,81182,81183,81184,83011,83012,83013,83014,83021,83022,83023,83024,83031,83032,83033,83034,83041,83042,83043,83044,83051,83052,83053,83054,83061,83062,83063,83064,83071,83072,83073,83074,83081,83082,83083,83084,83091,83092,83093,83094,83101,83102,83103,83104,83111,83112,83113,83114,83121,83122,83123,83124,83131,83132,83133,83134,83141,83142,83143,83144,89001,89002,89003,89004,100000,100001,110101,110111,110112,110113,110121,110122,110123,110131,110132,110133,110141,110142,110143,110151,110152,110153,110161,110162,110163,110171,110172,110173,110181,110182,110183,110211,110212,110213,110221,110222,110223,110231,110232,110233,110241,110242,110243,110400,110401,110402,110403,110404,110405,110406,110407,110411,110412,110413,110414,110415,110416,110417,110422,110423,110426,110501,110502,110503,110504,111001,111002,111003,111011,111012,111013,112001,112002,112003,112011,112012,112013,113001,113002,113003,113011,113012,113013,114001,114002,114003,114011,114012,114013,120000,120001,120002,120003,121000,121001,122000,140000,140004,140017,140021,140022,140023,140024,140025,140026,140027,140032,140033,140034,140035,140036,140037,140039,140040,140041,140046,140065,140066,140067,140068,140069,140070,140071,140072,140073,140074,140075,140076,140077,140078,140079,140080,140081,140082,140083,140084,140085,140086,140090,140091,140095,140096,140097,140098,140103,140104,140105,140106,140107,140108,140109,140110,140111,140112,140114,140115,140116,140117,140118,140119,140120,140121,140123,140124,140125,140126,140127,140128,140129,140130,140132,140133,140134,140135,140136,140137,140138,140139,140140,140141,140146,140147,140148,140149,140150,140151,140152,140153,140154,140155,140156,140157,140158,140159,140161,140162,140163,140164,140165,140166,140167,140168,140169,140170,140171,140172,140173,140174,140175,140176,140177,140178,140179,140180,140181,140182,140183,140184,140185,140186,140187,140188,140189,140190,140191,140192,140193,140194,140195,140196,140197,140198,140199,140200,140201,140202,140203,140206,140207,140208,140209,140210,140211,140212,140213,140214,140215,140216,140217,140218,140219,140220,140221,140222,140223,140225,140228,140229,140230,140231,140232,140233,140234,140235,140237,140239,140240,140241,140242,140243,140244,140245,140246,140247,140248,140249,140250,140251,140252,140253,140254,140255,140256,140257,140258,140259,140260,140261,140262,140263,140264,140265,140266,140267,140268,140269,140270,140271,140272,140273,140274,140275,140276,140277,140278,140279,140280,140281,140282,140283,140284,140285,140286,140287,140288,140289,140290,140291,140292,140293,140294,140295,140296,140297,140298,140299,140300,140301,140302,140303,140304,140305,140306,140307,140308,140309,140310,140311,140312,140313,140314,140315,140316,140317,140318,140319,140320,140321,140322,140323,140324,140325,140326,140327,140328,140329,140330,140331,140332,140333,140334,140335,140336,140339,140340,140341,140342,140343,140344,140345,140346,140347,140348,140349,140350,140351,140352,140353,140354,140355,140356,140357,140358,140359,140360,140361,140362,140363,140364,140365,140366,140367,140370,140372,140373,140374,140376,140377,140378,140380,140381,140382,140383,140384,140385,140386,140387,140388,140389,140390,140391,140392,140393,140394,140395,140396,140397,140398,140399,140400,140401,140402,140403,140404,140405,140406,140407,140408,140409,140410,140411,140412,140414,140415,140416,140417,140418,140419,140420,140421,140422,140423,140424,140425,140426,140427,140428,140429,140430,140431,140432,140433,140434,140435,140436,140437,140438,140439,140440,140441,140442,140443,140444,140445,140446,140447,140448,140449,140450,140451,140452,140453,140454,140455,140456,140457,140458,140459,140460,140461,140462,140463,140464,140465,140466,140467,140468,140469,140470,140471,140472,140473,140474,140475,140476,149957,149958,149959,149960,149961,149962,149963,149964,149965,149966,149967,149975,149976,149977,149978,149979,149980,149981,149982,149983,149984,149985,149986,149987,149988,149989,149990,149991,149997,149998,149999,150000,150001,150002,150003,150004,150010,150011,150012,150013,150014,150015,150016,150017,150018,150019,150020,150021,150022,150023,150024,150025,150026,150027,150028,150029,150030,150031,150032,150033,150034,150035,150039,180001,180002,180004,180005,180006,180007,180008,180009,180010,180011,180012,181001,181002,181003,181004,181005,181006,181007,181008,181009,181010,181011,181012,181013,181014,181015,181016,182001,182002,182003,182004,182005,182006,182007,182008,182009,182010,183001,183002,183003,183004,183005,183006,190101,190102,190103,190104,190105,190106,190107,190108,190109,190110,190111,190112,190113,190114,190115,190116,190117,190118,190119,190120,190121,190122,190123,190124,190125,190126,190127,190128,190129,190130,190131,190132,190133,190134,190135,190136,190137,190138,190139,190140,190141,190142,190143,190144,190145,190146,190147,190148,190149,190150,190151,190152,190153,190154,190155,190156,190157,190158,190159,190160,190161,190162,190163,190164,190165,190166,190167,190168,190169,190170,190171,190172,190173,190174,190175,190176,190177,190178,190179,190180,190181,190182,190183,190184,190185,190186,190187,190188,190189,190190,190191,190192,190193,190194,190195,190196,190197,190198,190199,190200,190201,190202,190203,190204,190205,190206,190207,190208,190209,190210,190211,190212,190213,190214,190215,190216,190217,190218,190219,190220,190221,190222,190223,190224,190225,190226,190227,190228,190229,190230,190231,190232,190233,190234,190235,190236,190237,190238,190239,190240,190241,190242,190243,190244,190245,190246,190247,190248,190249,190250,190251,190252,190253,190254,190255,190256,190257,190258,190259,190260,190261,190262,190263,190264,190265,190266,190267,190268,190269,190270,190271,190272,190273,190274,190275,190276,190277,190278,190279,190280,190281,190282,190283,190284,190285,190286,190287,190288,190289,190290,190291,190292,190293,190294,190295,190296,190297,190298,190299,190300,190301,190302,190303,190304,190305,190306,190307,190308,190309,190310,190311,190312,190313,190314,190315,190316,190317,190318,190319,190320,190321,190322,190323,190325,190327,190330,190331,190332,190333,190334,190335,190336,190337,190338,190339,190340,190341,190342,190343,190344,190345,190346,190347,190348,190349,190350,190351,190352,190353,190354,190355,190356,190357,190358,190359,190360,190361,190362,190363,190364,190365,190367,190368,190369,190370,190371,190372,190373,190374,190375,190376,190377,190378,190379,190380,190381,190382,190383,190384,190385,190386,190387,190388,190389,190390,190391,190392,190393,190394,190395,190396,190397,190398,190399,190400,190401,190402,190403,190404,190405,190406,190407,190408,190409,190410,190411,190412,190413,190414,190415,190416,190417,190901,190902,190903,190904,190905,190906,190907,191001,191002,191003,191004,191005,191006,191007,191008,191009,191010,191011,191012,191013,191014,191015,191016,191017,191018,191019,191020,191021,191022,191023,191024,191025,191026,191027,191028,191029,191030,191031,191032,191033,191034,191035,191036,220000,220001,220002,220003,220004,221000,221001,221002,221003,221004,221005,222000,223000,223001,223002,223003,223004,223005,223006,223007,223008,223009,223010,223020,223021,223022,223023,223024,223050,223051,223052,223053,223054,223055,223056,223057,223058,223059,223080,223081,223082,223083,223084,223085,223100,223101,223102,223103,223104,223105,223106,223107,223108,223109,223150,223151,223152,223153,223154,223155,223156,223157,223180,223181,223182,223183,223184,223185,223200,223201,223202,223203,223204,223205,223206,223207,223208,223230,223231,223232,223233,223250,223251,223252,223253,223254,223255,223256,223257,223258,223259,223260,223261,223270,223271,223272,223273,223274,223300,223301,223302,223303,223304,223305,223306,223307,223308,223309,223330,223331,223332,223333,223334,223350,223351,223352,223353,223354,223355,223356,223357,223358,223370,223371,223372,223373,223400,223401,223402,223403,223404,223405,223406,223407,223408,223409,223410,223411,223430,223431,223432,223433,223434,223450,223451,223452,223453,223454,223455,223456,223457,223500,223501,223502,223503,223504,223505,223506,223507,223508,223509,223531,223532,223533,223534,223600,223601,223602,223603,223604,223605,223606,223607,223631,223632,223633,223634,223700,223701,223702,223703,223704,223705,223706,223708,223710,223712,223713,223714,223731,223732,223733,223734,223735,250001,250005,250006,250007,250008,250009,250011,250012,250013,250015,250019,250020,250021,250022,250023,250027,250031,250032,250033,250034,250037,250101,250102,250103,250104,250105,250106,250107,250108,250109,250110,250111,250112,250113,250114,250115,250116,250117,250118,250119,250120,250121,250122,250123,250124,250125,250126,250127,250128,250129,250130,250131,250132,250133,250201,250202,250203,250204,250205,250207,250208,250209,250210,250211,250212,250213,250214,250215,250216,250217,250218,250219,250220,250221,250222,250223,250224,250225,250226,250227,250228,250229,250230,250231,250232,250233,250300,250302,250304,250305,250306,250307,250308,250309,250310,250311,250312,250313,250314,250315,250316,250317,250318,250319,250320,250321,250401,250402,250403,250404,250500,250501,250502,250503,250504,250505,250506,250507,250508,250509,250510,250511,250512,250513,250514,250515,250516,250517,250518,250519,250520,250521,250522,250523,250524,250525,250526,250527,250528,250529,250530,250531,250532,250533,250534,250535,250536,250537,250538,250539,250540,250541,250542,250543,250544,250545,250546,250547,250548,250549,250550,250551,250552,250553,250554,250555,250556,250557,250558,250559,250560,250561,250562,250563,250564,250565,250566,250567,250568,250569,250570,250571,250572,250573,250574,250575,250576,250577,250578,250579,250580,250581,250582,250583,250584,250585,250586,250587,250588,250589,250590,250591,250592,250593,250594,250595,250596,250597,250598,250599,270000,270001,270002,281001,281002,281003,281004,281005,281012,281013,281014,281015,281016,281017,300002,300003,300011,300020,300021,300031,300041,300042,300051,300052,300101,300102,300103,300104,300200,300201,300202,309001,309002,309003,310001,310002,310003,310004,310005,310006,310007,310008,310009,310010,310011,400004,400005,400006,400007,400008,401003,401004,401005,401006,401007,401008,401009,401010,401011,401012,401013,401014,401015,401016,401017,401018,401019,401020,401021,401022,401023,401024,401025,401026,401027,401028,402001,402002,402003,402004,402005,402006,402007,402008,402009,402010,402011,402012,402013,402014,402015,402016,402017,402018,402019,402020,402021,402022,402023,402024,402025,402026,402027,402028,403001,403002,403003,403004,403005,403006,403007,403008,403009,403010,404004,404005,404006,404007,404008,405003,405004,405005,405006,405007,405008,405009,405010,405011,405012,405013,405014,405015,405016,405017,405018,405019,405020,405021,405022,405023,405024,405025,405026,405027,405028,406001,406002,406003,406004,406005,406006,406007,406008,406009,406010,406011,406012,406013,406014,406015,406016,406017,406018,406019,406020,406021,406022,406023,406024,406025,406026,406027,406028,407001,407002,407003,407004,407005,407006,407007,407008,407009,407010,408001,408002,408003,408004,408005,408201,408202,408203,408204,408205,408206,408207,408208,408401,408402,408403,408404,408405,408406,408407,408408,408409,408410,408411,408412,408413,408414,408415,408416,408417,408418,408601,408602,408603,408604,408605,408606,408607,408608,408609,408610,408611,408612,408613,408614,408615,408616,408617,408618,408619,408620,408621,408622,408623,408801,408802,1,2,3,101,102,201,211,212,213,221,222,223,231,232,233,234,235,236,241,261,271,503,800,801,802,110101,110111,110112,110113,110121,110122,110123,110131,110132,110133,110141,110142,110143,110151,110152,110153,110161,110162,110163,110171,110172,110173,110181,110182,110183,110191,110192,110193,110211,110212,110213,110221,110222,110223,110231,110232,110233,110241,110242,110243,110400,110401,110402,110403,110404,110405,110406,110407,110411,110412,110413,110414,110415,110416,110417,110421,110422,110423,110426,110501,110502,110503,110504,110505,111001,111002,111003,111011,111012,111013,112001,112002,112003,112011,112012,112013,113001,113002,113003,113011,113012,113013,114001,114002,114003,114011,114012,114013,120000,120001,120002,120003,121000,121001,121002,122000,140000,140004,140017,140021,140022,140023,140024,140025,140026,140027,140032,140033,140034,140035,140036,140037,140039,140040,140041,140046,140065,140066,140067,140068,140069,140070,140071,140072,140073,140074,140075,140076,140077,140078,140079,140080,140081,140082,140083,140084,140085,140086,140090,140091,140095,140096,140097,140098,140103,140104,140105,140106,140107,140108,140109,140110,140111,140112,140114,140115,140116,140117,140118,140119,140120,140121,140123,140124,140125,140126,140127,140128,140129,140130,140132,140133,140134,140135,140136,140137,140138,140139,140140,140141,140146,140147,140148,140149,140150,140151,140152,140153,140154,140155,140156,140157,140158,140159,140161,140162,140163,140164,140165,140166,140167,140168,140169,140170,140171,140172,140173,140174,140175,140176,140177,140178,140179,140180,140181,140182,140183,140184,140185,140186,140187,140188,140189,140190,140191,140192,140193,140194,140195,140196,140197,140198,140199,140200,140201,140202,140203,140206,140207,140208,140209,140210,140211,140212,140213,140214,140215,140216,140217,140218,140219,140220,140221,140222,140223,140225,140228,140229,140230,140231,140232,140233,140234,140235,140237,140239,140240,140241,140242,140243,140244,140245,140246,140247,140248,140249,140250,140251,140252,140253,140254,140255,140256,140257,140258,140259,140260,140261,140262,140263,140264,140265,140266,140267,140268,140269,140270,140271,140272,140273,140274,140275,140276,140277,140278,140279,140280,140281,140282,140283,140284,140285,140286,140287,140288,140289,140290,140291,140292,140293,140294,140295,140296,140297,140298,140299,140300,140301,140302,140303,140304,140305,140306,140307,140308,140309,140310,140311,140312,140313,140314,140315,140316,140317,140318,140319,140320,140321,140322,140323,140324,140325,140326,140327,140328,140329,140330,140331,140332,140333,140334,140335,140336,140339,140340,140341,140342,140343,140344,140345,140346,140347,140348,140349,140350,140351,140352,140353,140354,140355,140356,140357,140358,140359,140360,140361,140362,140363,140364,140365,140366,140367,140370,140372,140373,140374,140376,140377,140378,140380,140381,140382,140383,140384,140385,140386,140387,140388,140389,140390,140391,140392,140393,140394,140395,140396,140397,140398,140399,140400,140401,140402,140403,140404,140405,140406,140407,140408,140409,140410,140411,140412,140414,140415,140416,140417,140418,140419,140420,140421,140422,140423,140424,140425,140426,140427,140428,140429,140430,140431,140432,140433,140434,140435,140436,140437,140438,140439,140440,140441,140442,140443,140444,140445,140446,140447,140448,140449,140450,140451,140452,140453,140454,140455,140456,140457,140458,140459,140460,140461,140462,140463,140464,140465,140466,140467,140468,140469,140470,140471,140472,140473,140474,140475,140476,140477,140478,140479,140483,140484,140485,140486,140487,140488,140492,140493,140494,140495,149957,149958,149959,149960,149961,149962,149963,149964,149965,149966,149967,149975,149976,149977,149978,149979,149980,149981,149982,149983,149984,149985,149986,149987,149988,149989,149990,149991,149997,149998,149999,150000,150001,150002,150003,150004,150010,150011,150012,150013,150014,150015,150016,150017,150018,150019,150020,150021,150022,150023,150024,150025,150026,150027,150028,150029,150030,150031,150032,150033,150034,150035,150036,150037,150038,150039,150040,150041,150042,150043,150044,150045,150046,180001,180002,180004,180005,180006,180007,180008,180009,180010,180011,180012,181001,181002,181003,181004,181005,181006,181007,181008,181009,181010,181011,181012,181013,181014,181015,181016,182001,182002,182003,182004,182005,182006,182007,182008,182009,182010,183001,183002,183003,183004,183005,183006,190101,190102,190103,190104,190105,190106,190107,190108,190109,190110,190111,190112,190113,190114,190115,190116,190117,190118,190119,190120,190121,190122,190123,190124,190125,190126,190127,190128,190129,190130,190131,190132,190133,190134,190135,190136,190137,190138,190139,190140,190141,190142,190143,190144,190145,190146,190147,190148,190149,190150,190151,190152,190153,190154,190155,190156,190157,190158,190159,190160,190161,190162,190163,190164,190165,190166,190167,190168,190169,190170,190171,190172,190173,190174,190175,190176,190177,190178,190179,190180,190181,190182,190183,190184,190185,190186,190187,190188,190189,190190,190191,190192,190193,190194,190195,190196,190197,190198,190199,190200,190201,190202,190203,190204,190205,190206,190207,190208,190209,190210,190211,190212,190213,190214,190215,190216,190217,190218,190219,190220,190221,190222,190223,190224,190225,190226,190227,190228,190229,190230,190231,190232,190233,190234,190235,190236,190237,190238,190239,190240,190241,190242,190243,190244,190245,190246,190247,190248,190249,190250,190251,190252,190253,190254,190255,190256,190257,190258,190259,190260,190261,190262,190263,190264,190265,190266,190267,190268,190269,190270,190271,190272,190273,190274,190275,190276,190277,190278,190279,190280,190281,190282,190283,190284,190285,190286,190287,190288,190289,190290,190291,190292,190293,190294,190295,190296,190297,190298,190299,190300,190301,190302,190303,190304,190305,190306,190307,190308,190309,190310,190311,190312,190313,190314,190315,190316,190317,190318,190319,190320,190321,190322,190323,190325,190327,190330,190331,190332,190333,190334,190335,190336,190337,190338,190339,190340,190341,190342,190343,190344,190345,190346,190347,190348,190349,190350,190351,190352,190353,190354,190355,190356,190357,190358,190359,190360,190361,190362,190363,190364,190365,190367,190368,190369,190370,190371,190372,190373,190374,190375,190376,190377,190378,190379,190380,190381,190382,190383,190384,190385,190386,190387,190388,190389,190390,190391,190392,190393,190394,190395,190396,190397,190398,190399,190400,190401,190402,190403,190404,190405,190406,190407,190408,190409,190410,190411,190412,190413,190414,190415,190416,190417,190901,190902,190903,190904,190905,190906,190907,191001,191002,191003,191004,191005,191006,191007,191008,191009,191010,191011,191012,191013,191014,191015,191016,191017,191018,191019,191020,191021,191022,191023,191024,191025,191026,191027,191028,191029,191030,191031,191032,191033,191034,191035,191036,250401,250402,250403,250404,300021,300031,300041,300042,300051,300052,300101,300102,300103,300104,400004,400005,400006,400007,400008,401003,401004,401005,401006,401007,401008,401009,401010,401011,401012,401013,401014,401015,401016,401017,401018,401019,401020,401021,401022,401023,401024,401025,401026,401027,401028,402001,402002,402003,402004,402005,402006,402007,402008,402009,402010,402011,402012,402013,402014,402015,402016,402017,402018,402019,402020,402021,402022,402023,402024,402025,402026,402027,402028,403001,403002,403003,403004,403005,403006,403007,403008,403009,403010,403011,404004,404005,404006,404007,404008,405003,405004,405005,405006,405007,405008,405009,405010,405011,405012,405013,405014,405015,405016,405017,405018,405019,405020,405021,405022,405023,405024,405025,405026,405027,405028,406001,406002,406003,406004,406005,406006,406007,406008,406009,406010,406011,406012,406013,406014,406015,406016,406017,406018,406019,406020,406021,406022,406023,406024,406025,406026,406027,406028,407001,407002,407003,407004,407005,407006,407007,407008,407009,407010,407011,408001,408002,408003,408004,408005,408201,408202,408203,408204,408205,408206,408207,408208,408401,408402,408403,408404,408405,408406,408407,408408,408409,408410,408411,408412,408413,408414,408415,408416,408417,408418,408419,408420,408421,408422,408601,408602,408603,408604,408605,408606,408607,408608,408609,408610,408611,408612,408613,408614,408615,408616,408617,408618,408619,408620,408621,408622,408623,408624,408625,408626,408627,408801,408802];