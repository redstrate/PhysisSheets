#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GCSupplyDutyReward {
exd: EXD,
exh: EXH,
}
impl GCSupplyDutyReward {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("GCSupplyDutyReward")?;let exd = game_data.read_excel_sheet("GCSupplyDutyReward", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<GCSupplyDutyRewardRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GCSupplyDutyRewardRow { columns })
}
}
pub struct GCSupplyDutyRewardRow {
columns: Vec<ColumnData>,
}
impl GCSupplyDutyRewardRow {
pub fn ExperienceSupply(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ExperienceProvisioning(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SealsExpertDelivery(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SealsSupply(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SealsProvisioning(&self) -> &ColumnData {
&self.columns[4]
}
}
