#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FateProgressUI {
exd: EXD,
exh: EXH,
}
impl FateProgressUI {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FateProgressUI").unwrap();let exd = game_data.read_excel_sheet("FateProgressUI", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FateProgressUIRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FateProgressUIRow { columns: row.columns.clone() }
}
}
pub struct FateProgressUIRow {
columns: Vec<ColumnData>,
}
impl FateProgressUIRow {
pub fn Location(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ReqFatesToRank2(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ReqFatesToRank3(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ReqFatesToRank4(&self) -> &ColumnData {
&self.columns[3]
}
pub fn DisplayOrder(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
}
