#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Quest {
exd: EXD,
exh: EXH,
}
impl Quest {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Quest")?;let exd = game_data.read_excel_sheet("Quest", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<QuestRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(QuestRow { columns })
}
}
pub struct QuestRow {
columns: Vec<ColumnData>,
}
impl QuestRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn QuestParams(&self) -> &ColumnData {
&self.columns[1]
}
pub fn QuestListenerParams(&self) -> &ColumnData {
&self.columns[2]
}
pub fn TodoParams(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GilReward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CurrencyReward(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CurrencyRewardCount(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Reward(&self) -> &ColumnData {
&self.columns[7]
}
pub fn OptionalItemReward(&self) -> &ColumnData {
&self.columns[8]
}
pub fn InstanceContentUnlock(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ExpFactor(&self) -> &ColumnData {
&self.columns[10]
}
pub fn EmoteReward(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ActionReward(&self) -> &ColumnData {
&self.columns[12]
}
pub fn SystemReward(&self) -> &ColumnData {
&self.columns[13]
}
pub fn GCTypeReward(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ItemCatalyst(&self) -> &ColumnData {
&self.columns[15]
}
pub fn ItemCountCatalyst(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ItemRewardType(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ItemCountReward(&self) -> &ColumnData {
&self.columns[18]
}
pub fn RewardStain(&self) -> &ColumnData {
&self.columns[19]
}
pub fn OptionalItemCountReward(&self) -> &ColumnData {
&self.columns[20]
}
pub fn OptionalItemStainReward(&self) -> &ColumnData {
&self.columns[21]
}
pub fn GeneralActionReward(&self) -> &ColumnData {
&self.columns[22]
}
pub fn OtherReward(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Tomestone(&self) -> &ColumnData {
&self.columns[24]
}
pub fn TomestoneReward(&self) -> &ColumnData {
&self.columns[25]
}
pub fn TomestoneCountReward(&self) -> &ColumnData {
&self.columns[26]
}
pub fn ReputationReward(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[34]
}
pub fn OptionalItemIsHQReward(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Id(&self) -> &ColumnData {
&self.columns[36]
}
pub fn PreviousQuest(&self) -> &ColumnData {
&self.columns[37]
}
pub fn QuestLock(&self) -> &ColumnData {
&self.columns[38]
}
pub fn InstanceContent(&self) -> &ColumnData {
&self.columns[39]
}
pub fn IssuerStart(&self) -> &ColumnData {
&self.columns[40]
}
pub fn IssuerLocation(&self) -> &ColumnData {
&self.columns[41]
}
pub fn TargetEnd(&self) -> &ColumnData {
&self.columns[42]
}
pub fn JournalGenre(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[44]
}
pub fn IconSpecial(&self) -> &ColumnData {
&self.columns[45]
}
pub fn MountRequired(&self) -> &ColumnData {
&self.columns[46]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Header(&self) -> &ColumnData {
&self.columns[48]
}
pub fn BellStart(&self) -> &ColumnData {
&self.columns[49]
}
pub fn BellEnd(&self) -> &ColumnData {
&self.columns[50]
}
pub fn BeastReputationValue(&self) -> &ColumnData {
&self.columns[51]
}
pub fn ClientBehavior(&self) -> &ColumnData {
&self.columns[52]
}
pub fn QuestClassJobSupply(&self) -> &ColumnData {
&self.columns[53]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[54]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[55]
}
pub fn Expansion(&self) -> &ColumnData {
&self.columns[56]
}
pub fn ClassJobCategory0(&self) -> &ColumnData {
&self.columns[57]
}
pub fn QuestLevelOffset(&self) -> &ColumnData {
&self.columns[58]
}
pub fn ClassJobCategory1(&self) -> &ColumnData {
&self.columns[59]
}
pub fn PreviousQuestJoin(&self) -> &ColumnData {
&self.columns[60]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[61]
}
pub fn QuestLockJoin(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[63]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[64]
}
pub fn ClassJobUnlock(&self) -> &ColumnData {
&self.columns[65]
}
pub fn GrandCompany(&self) -> &ColumnData {
&self.columns[66]
}
pub fn GrandCompanyRank(&self) -> &ColumnData {
&self.columns[67]
}
pub fn InstanceContentJoin(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Festival(&self) -> &ColumnData {
&self.columns[69]
}
pub fn FestivalBegin(&self) -> &ColumnData {
&self.columns[70]
}
pub fn FestivalEnd(&self) -> &ColumnData {
&self.columns[71]
}
pub fn BeastTribe(&self) -> &ColumnData {
&self.columns[72]
}
pub fn BeastReputationRank(&self) -> &ColumnData {
&self.columns[73]
}
pub fn SatisfactionNpc(&self) -> &ColumnData {
&self.columns[74]
}
pub fn SatisfactionLevel(&self) -> &ColumnData {
&self.columns[75]
}
pub fn DeliveryQuest(&self) -> &ColumnData {
&self.columns[76]
}
pub fn RepeatIntervalType(&self) -> &ColumnData {
&self.columns[77]
}
pub fn QuestRepeatFlag(&self) -> &ColumnData {
&self.columns[78]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[79]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[80]
}
pub fn LevelMax(&self) -> &ColumnData {
&self.columns[81]
}
pub fn ClassJobRequired(&self) -> &ColumnData {
&self.columns[82]
}
pub fn QuestRewardOtherDisplay(&self) -> &ColumnData {
&self.columns[83]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[84]
}
pub fn EventIconType(&self) -> &ColumnData {
&self.columns[85]
}
pub fn DailyQuestPool(&self) -> &ColumnData {
&self.columns[86]
}
pub fn IsHouseRequired(&self) -> &ColumnData {
&self.columns[87]
}
pub fn IsRepeatable(&self) -> &ColumnData {
&self.columns[88]
}
pub fn CanCancel(&self) -> &ColumnData {
&self.columns[89]
}
pub fn Introduction(&self) -> &ColumnData {
&self.columns[90]
}
pub fn HideOfferIcon(&self) -> &ColumnData {
&self.columns[91]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[92]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[93]
}
}
