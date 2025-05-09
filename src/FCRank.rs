#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCRank {
exd: EXD,
exh: EXH,
}
impl FCRank {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCRank").unwrap();let exd = game_data.read_excel_sheet("FCRank", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCRankRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCRankRow { columns: row.columns.clone() }
}
}
pub struct FCRankRow {
columns: Vec<ColumnData>,
}
impl FCRankRow {
pub fn NextPoint(&self) -> &ColumnData {
&self.columns[0]
}
pub fn CurrentPoint(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Rights(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn FCActionActiveNum(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FCActionStockNum(&self) -> &ColumnData {
&self.columns[6]
}
pub fn FCChestCompartments(&self) -> &ColumnData {
&self.columns[7]
}
}
