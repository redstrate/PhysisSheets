#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SatisfactionSupplyRewardSheet {
exd: EXD,
exh: EXH,
}
impl SatisfactionSupplyRewardSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("SatisfactionSupplyReward")?;let exd = game_data.read_excel_sheet("SatisfactionSupplyReward", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<SatisfactionSupplyRewardRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SatisfactionSupplyRewardRow { columns })
}
}
pub struct SatisfactionSupplyRewardRow {
columns: Vec<ColumnData>,
}
impl SatisfactionSupplyRewardRow {
pub fn SatisfactionSupplyRewardData(&self) -> [&ColumnData; 2] {
[&self.columns[0],&self.columns[1],]
}
pub fn SatisfactionLow(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SatisfactionMid(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SatisfactionHigh(&self) -> &ColumnData {
&self.columns[4]
}
pub fn GilLow(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GilMid(&self) -> &ColumnData {
&self.columns[6]
}
pub fn GilHigh(&self) -> &ColumnData {
&self.columns[7]
}
pub fn BonusMultiplier(&self) -> &ColumnData {
&self.columns[8]
}
pub fn MinLevelForSecondReward(&self) -> &ColumnData {
&self.columns[9]
}
}
