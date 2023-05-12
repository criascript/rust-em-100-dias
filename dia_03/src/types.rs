use std::collections::BTreeMap;

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    #[serde(rename = "AppContext")]
    pub app_context: AppContext,
    #[serde(rename = "BizContext")]
    pub biz_context: BizContext,
    #[serde(rename = "SEOState")]
    pub seostate: Seostate,
    #[serde(rename = "ItemList")]
    pub item_list: ItemList,
    #[serde(rename = "ItemModule")]
    pub item_module: BTreeMap<String, VideoData>,
    #[serde(rename = "UserModule")]
    pub user_module: UserModule,
    #[serde(rename = "UserPage")]
    pub user_page: UserPage,
    #[serde(rename = "RecommendUserList")]
    pub recommend_user_list: RecommendUserList,
    #[serde(rename = "PlaylistItemModule")]
    pub playlist_item_module: PlaylistItemModule,
    #[serde(rename = "VideoPlaylistSharedModule")]
    pub video_playlist_shared_module: VideoPlaylistSharedModule,
    #[serde(rename = "SharingMetaState")]
    pub sharing_meta_state: SharingMetaState,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppContext {
    pub app_context: AppContextData,
    pub initialized: bool,
    pub lang: String,
    pub side_nav_active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppContextData {
    pub language: String,
    pub region: String,
    pub app_id: i64,
    pub app_type: String,
    pub wid: String,
    pub nonce: String,
    pub bot_type: String,
    pub request_id: String,
    pub cluster_region: String,
    pub ab_test_version: AbTestVersion,
    pub csrf_token: String,
    pub user_agent: String,
    pub encrypted_webid: String,
    pub host: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbTestVersion {
    pub version_name: String,
    pub parameters: BTreeMap<String, Param>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BizContext {
    pub biz_context: BizContextData,
    pub initialized: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BizContextData {
    pub os: String,
    pub is_mobile: bool,
    pub is_android: bool,
    #[serde(rename = "isIOS")]
    pub is_ios: bool,
    pub jump_type: String,
    pub nav_list: Vec<NavList>,
    pub config: Config,
    pub domains: Domains,
    pub download_link: DownloadLink,
    pub device_limit_register_expired: bool,
    pub subdivisions: Vec<String>,
    pub geo: Vec<String>,
    pub geo_city: GeoCity,
    pub is_google_bot: bool,
    pub is_bing_bot: bool,
    pub is_bot: bool,
    pub is_search_engine_bot: bool,
    #[serde(rename = "isTTP")]
    pub is_ttp: bool,
    pub date_fmt_locale: DateFmtLocale,
    pub video_player_config: VideoPlayerConfig,
    pub playback_normalize_path: PlaybackNormalizePath,
    pub bitrate_config: BitrateConfig,
    pub search_video_for_loggedin: bool,
    pub studio_download_entrance: StudioDownloadEntrance,
    pub live_suggest_config: LiveSuggestConfig,
    pub live_anchor_entrance: LiveAnchorEntrance,
    pub live_studio_enable: bool,
    pub xgplayer_init_host: XgplayerInitHost,
    pub video_order: VideoOrder,
    pub search_live_for_loggedin: bool,
    pub can_use_query: bool,
    pub bitrate_selector_configs: BitrateSelectorConfigs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavList {
    pub title: String,
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub title: String,
    pub href: String,
    pub key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub feature_flags: FeatureFlags,
    pub sign_up_open: bool,
    pub cookie_banner: CookieBanner,
    pub is_gray_filter: bool,
    pub nick_name_control_day: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlags {
    #[serde(rename = "feature_bar")]
    pub feature_bar: bool,
    #[serde(rename = "business_account_open")]
    pub business_account_open: bool,
    #[serde(rename = "feature_tt4b_ads")]
    pub feature_tt4b_ads: bool,
    #[serde(rename = "support_multiline_desc")]
    pub support_multiline_desc: bool,
    #[serde(rename = "pc_video_playlist")]
    pub pc_video_playlist: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieBanner {
    #[serde(rename = "load_dynamically")]
    pub load_dynamically: bool,
    #[serde(rename = "decline_btn_staged_rollout_area")]
    pub decline_btn_staged_rollout_area: Vec<String>,
    pub resource: Resource,
    pub i18n: I18n,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub prefix: String,
    pub themes: Vec<String>,
    pub esm: String,
    pub nomodule: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18n {
    pub cookie_banner_title: String,
    pub cookie_banner_title_new: String,
    pub cookie_banner_sub_title: String,
    pub cookie_banner_sub_title_new: String,
    pub cookie_banner_sub_title_v2: String,
    pub cookie_banner_btn_manage: String,
    pub cookie_banner_btn_accept: String,
    pub cookie_banner_btn_decline: String,
    pub cookies_banner_details: String,
    pub cookies_banner_cookies_policy: String,
    pub cookies_banner_accept: String,
    pub web_do_not_sell_settings_saved_toast: String,
    pub cookie_setting_manage_your_cookie_title: String,
    pub cookie_setting_save: String,
    pub cookie_setting_analytics_and_marketing: String,
    pub cookie_setting_necessary: String,
    pub cookie_setting_necessary_subtitle: String,
    pub cookie_setting_necessary_v2: String,
    pub cookie_setting_necessary_subtitle_v2: String,
    pub cookie_setting_analytics_and_marketing_subtitle: String,
    pub cookie_setting_analytics_and_marketing_subtitle_v2: String,
    pub cookie_manage_tip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domains {
    pub kind: String,
    pub captcha: String,
    pub im_api: String,
    pub im_frontier: String,
    #[serde(rename = "mTApi")]
    pub m_tapi: String,
    pub root_api: String,
    #[serde(rename = "secSDK")]
    pub sec_sdk: String,
    pub slardar: String,
    pub starling: String,
    pub tea: String,
    #[serde(rename = "libraWebSDK")]
    pub libra_web_sdk: String,
    pub webcast_api: String,
    pub webcast_root_api: String,
    pub pipo_api: String,
    pub tcc: String,
    pub search: String,
    pub aweme: String,
    pub location_api: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLink {
    pub microsoft: Microsoft,
    pub apple: Apple,
    pub amazon: Amazon,
    pub google: Google,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microsoft {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apple {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amazon {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Google {
    pub visible: bool,
    pub normal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoCity {
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "Subdivisions")]
    pub subdivisions: String,
    #[serde(rename = "OriginalSubdivisions")]
    pub original_subdivisions: Vec<OriginalSubdivision>,
    #[serde(rename = "SubdivisionsArr")]
    pub subdivisions_arr: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalSubdivision {
    #[serde(rename = "GeoNameID")]
    pub geo_name_id: String,
    #[serde(rename = "ASCIName")]
    pub asciname: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFmtLocale {
    pub name: String,
    pub months: Vec<String>,
    pub months_short: Vec<String>,
    pub weekdays: Vec<String>,
    pub weekdays_short: Vec<String>,
    pub weekdays_min: Vec<String>,
    pub long_date_format: LongDateFormat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongDateFormat {
    #[serde(rename = "LT")]
    pub lt: String,
    #[serde(rename = "LTS")]
    pub lts: String,
    #[serde(rename = "L")]
    pub l: String,
    #[serde(rename = "LL")]
    pub ll: String,
    #[serde(rename = "LLL")]
    pub lll: String,
    #[serde(rename = "LLLL")]
    pub llll: String,
    #[serde(rename = "l")]
    pub l2: String,
    #[serde(rename = "ll")]
    pub ll2: String,
    #[serde(rename = "lll")]
    pub lll2: String,
    #[serde(rename = "llll")]
    pub llll2: String,
    #[serde(rename = "LL-Y")]
    pub ll_y: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlayerConfig {
    pub fallback: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackNormalizePath {
    pub path: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitrateConfig {
    pub bitrate_lower: i64,
    pub bitrate_range: Vec<i64>,
    pub bitrate_upper: i64,
    pub mode: String,
    pub param_bf: f64,
    pub param_bp: f64,
    pub param_lower: f64,
    pub param_upper: f64,
    pub param_upper_bl: f64,
    pub param_vl1: f64,
    pub param_vl2: i64,
    pub param_vl_lower: f64,
    pub param_vl_upper: f64,
    pub sliding_window_count_threshold: i64,
    pub sliding_window_extraction: String,
    pub sliding_window_type: String,
    pub sliding_window_weight: String,
    pub sliding_window_weight_threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioDownloadEntrance {
    pub regions: Vec<String>,
    pub user_regions: Vec<String>,
    pub all_regions: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveSuggestConfig {
    pub is_blocked_area: bool,
    pub is_risk_area: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveAnchorEntrance {
    pub live_center: bool,
    pub creator_hub: bool,
    pub live_studio: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XgplayerInitHost {
    pub group1: Vec<String>,
    pub group2: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOrder {
    pub video_order: Vec<VideoOrderData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOrderData {
    pub property: String,
    pub detail: Option<Vec<i64>>,
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitrateSelectorConfigs {
    pub configs: Vec<ConfigData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigData {
    pub param_bf: f64,
    pub param_bp: f64,
    pub param_upper: f64,
    pub param_lower: f64,
    pub param_upper_bl: f64,
    pub param_vl1: f64,
    pub param_vl2: i64,
    pub param_vl_upper: f64,
    pub param_vl_lower: f64,
    pub bitrate_upper: i64,
    pub bitrate_lower: i64,
    pub sliding_window_type: String,
    pub sliding_window_weight: String,
    pub sliding_window_weight_threshold: i64,
    pub sliding_window_count_threshold: i64,
    pub sliding_window_extraction: String,
    pub bitrate_range: Vec<i64>,
    pub mode: String,
    #[serde(rename = "quality_filter")]
    pub quality_filter: QualityFilter,
    #[serde(rename = "white_list")]
    pub white_list: Vec<Value>,
    pub auto_bitrate_params: AutoBitrateParams,
    pub default_bitrate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityFilter {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoBitrateParams {
    pub param_a: i64,
    pub param_b: f64,
    pub param_c: f64,
    pub param_d: i64,
    pub min_bitrate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seostate {
    pub meta_params: MetaParams,
    pub jsonld_list: Vec<(String, JsonldList)>,
    pub abtest: Abtest,
    pub loading: bool,
    pub canonical: String,
    pub page_type: i64,
    pub launch_mode: String,
    pub traffic_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaParams {
    pub title: String,
    pub description: String,
    pub canonical_href: String,
    pub robots_content: String,
    pub applicable_device: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonldList {
    #[serde(default)]
    pub item_list_element: Vec<ItemListElement>,
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub alternate_name: Option<String>,
    pub url: Option<String>,
    pub interaction_statistic: Option<Vec<InteractionStatistic>>,
    pub main_entity_of_page: Option<MainEntityOfPage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemListElement {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub position: i64,
    pub item: Item,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionStatistic {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub interaction_type: InteractionType,
    pub user_interaction_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionType {
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainEntityOfPage {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abtest {
    pub page_id: String,
    pub vid_list: Vec<Value>,
    pub parameters: AbtestParams,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbtestParams {
    #[serde(rename = "user_page_serp_compliance")]
    pub user_page_serp_compliance: UserPageSerpCompliance,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPageSerpCompliance {
    pub vid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemList {
    pub favorites: Favorites,
    #[serde(rename = "user-post")]
    pub user_post: UserPost,
    #[serde(rename = "user-liked")]
    pub user_liked: UserLiked,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Favorites {
    pub list: Vec<Value>,
    pub browser_list: Vec<Value>,
    pub loading: bool,
    pub status_code: i64,
    pub has_more: bool,
    pub cursor: String,
    pub preload_list: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPost {
    pub list: Vec<String>,
    pub browser_list: Vec<String>,
    pub loading: bool,
    pub status_code: i64,
    pub has_more: bool,
    pub cursor: String,
    pub preload_list: Vec<PreloadList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreloadList {
    pub url: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLiked {
    pub list: Vec<Value>,
    pub browser_list: Vec<Value>,
    pub loading: bool,
    pub status_code: i64,
    pub has_more: bool,
    pub cursor: String,
    pub preload_list: Vec<Value>,
}

fn deserialize_from_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let timestamp = s.parse::<i64>().map_err(serde::de::Error::custom)?;
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoData {
    pub id: String,
    pub desc: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub create_time: DateTime<Utc>,
    pub schedule_time: i64,
    pub video: Video,
    pub author: String,
    pub music: Music,
    pub challenges: Vec<Challenge>,
    pub stats: VideoStats,
    pub duet_info: DuetInfo,
    pub warn_info: Vec<Value>,
    pub original_item: bool,
    pub offical_item: bool,
    pub text_extra: Vec<TextExtra>,
    pub secret: bool,
    pub for_friend: bool,
    pub digged: bool,
    pub item_comment_status: i64,
    pub show_not_pass: bool,
    pub vl1: bool,
    pub take_down: i64,
    pub item_mute: bool,
    pub effect_stickers: Vec<Value>,
    pub author_stats: AuthorStats,
    pub private_item: bool,
    pub duet_enabled: bool,
    pub stitch_enabled: bool,
    pub stickers_on_item: Vec<Value>,
    pub is_ad: bool,
    pub share_enabled: bool,
    pub comments: Vec<Value>,
    pub duet_display: i64,
    pub stitch_display: i64,
    pub index_enabled: bool,
    pub diversification_labels: Option<Vec<String>>,
    pub ad_authorization: bool,
    pub ad_label_version: i64,
    pub location_created: String,
    #[serde(rename = "BAInfo")]
    pub bainfo: String,
    pub contents: Vec<Content>,
    pub playlist_id: String,
    pub diversification_id: Option<i64>,
    pub collected: bool,
    pub channel_tags: Vec<Value>,
    pub nickname: String,
    pub author_id: String,
    pub author_sec_id: String,
    pub avatar_thumb: String,
    pub download_setting: i64,
    pub author_private: bool,
    pub capcut_anchors: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub height: i64,
    pub width: i64,
    pub duration: i64,
    pub ratio: String,
    pub cover: String,
    pub origin_cover: String,
    pub dynamic_cover: String,
    pub play_addr: String,
    pub download_addr: String,
    pub share_cover: Vec<String>,
    pub reflow_cover: String,
    pub bitrate: i64,
    pub encoded_type: String,
    pub format: String,
    pub video_quality: String,
    pub encode_user_tag: String,
    pub codec_type: String,
    pub definition: String,
    pub subtitle_infos: Vec<Value>,
    pub zoom_cover: ZoomCover,
    pub volume_info: VolumeInfo,
    pub bitrate_info: Vec<BitrateInfo>,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoomCover {
    #[serde(rename = "240")]
    pub n240: String,
    #[serde(rename = "480")]
    pub n480: String,
    #[serde(rename = "720")]
    pub n720: String,
    #[serde(rename = "960")]
    pub n960: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    #[serde(rename = "Loudness")]
    pub loudness: f64,
    #[serde(rename = "Peak")]
    pub peak: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitrateInfo {
    #[serde(rename = "GearName")]
    pub gear_name: String,
    #[serde(rename = "Bitrate")]
    pub bitrate: i64,
    #[serde(rename = "QualityType")]
    pub quality_type: i64,
    #[serde(rename = "PlayAddr")]
    pub play_addr: PlayAddr,
    #[serde(rename = "CodecType")]
    pub codec_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayAddr {
    #[serde(rename = "Uri")]
    pub uri: String,
    #[serde(rename = "UrlList")]
    pub url_list: Vec<String>,
    #[serde(rename = "DataSize")]
    pub data_size: String,
    #[serde(rename = "UrlKey")]
    pub url_key: String,
    #[serde(rename = "FileHash")]
    pub file_hash: String,
    #[serde(rename = "FileCs")]
    pub file_cs: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Music {
    pub id: String,
    pub title: String,
    pub play_url: String,
    pub cover_large: String,
    pub cover_medium: String,
    pub cover_thumb: String,
    pub author_name: String,
    pub original: bool,
    pub duration: i64,
    pub album: String,
    pub schedule_search_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub title: String,
    pub is_commerce: bool,
    pub stats: ChallengeStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeStats {
    pub video_count: i64,
    pub view_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStats {
    pub digg_count: i64,
    pub share_count: i64,
    pub comment_count: i64,
    pub play_count: i64,
    pub collect_count: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuetInfo {
    pub duet_from_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextExtra {
    pub aweme_id: String,
    pub start: i64,
    pub end: i64,
    pub hashtag_id: String,
    pub hashtag_name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub sub_type: i64,
    pub user_id: String,
    pub is_commerce: bool,
    pub user_unique_id: String,
    pub sec_uid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorStats {
    pub follower_count: i64,
    pub following_count: i64,
    pub heart: i64,
    pub heart_count: i64,
    pub video_count: i64,
    pub digg_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub desc: String,
    pub text_extra: Vec<TextExtra>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModule {
    pub users: BTreeMap<String, User>,
    pub stats: BTreeMap<String, UserStats>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub short_id: String,
    pub unique_id: String,
    pub nickname: String,
    pub avatar_larger: String,
    pub avatar_medium: String,
    pub avatar_thumb: String,
    pub signature: String,
    pub create_time: i64,
    pub verified: bool,
    pub sec_uid: String,
    pub ftc: bool,
    pub relation: i64,
    pub open_favorite: bool,
    pub comment_setting: i64,
    pub commerce_user_info: CommerceUserInfo,
    pub duet_setting: i64,
    pub stitch_setting: i64,
    pub private_account: bool,
    pub secret: bool,
    #[serde(rename = "isADVirtual")]
    pub is_advirtual: bool,
    pub room_id: String,
    pub unique_id_modify_time: i64,
    pub tt_seller: bool,
    pub region: String,
    pub download_setting: i64,
    pub profile_tab: ProfileTab,
    pub following_visibility: i64,
    pub recommend_reason: String,
    pub now_invitation_card_url: String,
    pub nick_name_modify_time: i64,
    pub is_embed_banned: bool,
    pub can_exp_playlist: bool,
    pub profile_embed_permission: i64,
    pub extra_info: ExtraInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommerceUserInfo {
    pub commerce_user: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileTab {
    pub show_music_tab: bool,
    pub show_question_tab: bool,
    pub show_play_list_tab: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraInfo {
    pub status_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub follower_count: i64,
    pub following_count: i64,
    pub heart: i64,
    pub heart_count: i64,
    pub video_count: i64,
    pub digg_count: i64,
    pub need_fix: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPage {
    pub unique_id: String,
    pub status_code: i64,
    pub sec_uid: String,
    pub profile_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendUserList {
    pub unique_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemModule {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlaylistSharedModule {
    pub loading: bool,
    pub change_order_loading: bool,
    pub has_more: bool,
    pub status_code: i64,
    pub play_list: Vec<Value>,
    pub cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharingMetaState {
    pub value: SharingMetaStateValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharingMetaStateValue {
    #[serde(rename = "al:ios:url")]
    pub al_ios_url: String,
    #[serde(rename = "al:android:url")]
    pub al_android_url: String,
    #[serde(rename = "al:ios:app_store_id")]
    pub al_ios_app_store_id: String,
    #[serde(rename = "al:ios:app_name")]
    pub al_ios_app_name: String,
    #[serde(rename = "al:android:app_name")]
    pub al_android_app_name: String,
    #[serde(rename = "al:android:package")]
    pub al_android_package: String,
    #[serde(rename = "og:site_name")]
    pub og_site_name: String,
    #[serde(rename = "og:type")]
    pub og_type: String,
    #[serde(rename = "og:title")]
    pub og_title: String,
    #[serde(rename = "og:description")]
    pub og_description: String,
    #[serde(rename = "fb:app_id")]
    pub fb_app_id: String,
    #[serde(rename = "twitter:app:id:iphone")]
    pub twitter_app_id_iphone: String,
    #[serde(rename = "twitter:app:id:googleplay")]
    pub twitter_app_id_googleplay: String,
    #[serde(rename = "twitter:card")]
    pub twitter_card: String,
    #[serde(rename = "twitter:site")]
    pub twitter_site: String,
    #[serde(rename = "twitter:title")]
    pub twitter_title: String,
    #[serde(rename = "twitter:description")]
    pub twitter_description: String,
    #[serde(rename = "og:image")]
    pub og_image: String,
    #[serde(rename = "twitter:image")]
    pub twitter_image: String,
    #[serde(rename = "og:image:width")]
    pub og_image_width: String,
    #[serde(rename = "og:image:height")]
    pub og_image_height: String,
    #[serde(rename = "og:image:alt")]
    pub og_image_alt: String,
}
