#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SatisfactionSupply {
exd: EXD,
exh: EXH,
}
impl SatisfactionSupply {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("SatisfactionSupply")?;let exd = game_data.read_excel_sheet("SatisfactionSupply", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<SatisfactionSupplyRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SatisfactionSupplyRow { columns })
}
}
pub struct SatisfactionSupplyRow {
columns: Vec<ColumnData>,
}
impl SatisfactionSupplyRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CollectabilityLow(&self) -> &ColumnData {
&self.columns[1]
}
pub fn CollectabilityMid(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CollectabilityHigh(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Reward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn FishingSpotId(&self) -> &ColumnData {
&self.columns[5]
}
pub fn SpearFishingSpotId(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ProbabilityPercent(&self) -> &ColumnData {
&self.columns[8]
}
pub fn IsBonus(&self) -> &ColumnData {
&self.columns[9]
}
}
