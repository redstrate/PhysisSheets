#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DawnGrowMember {
exd: EXD,
exh: EXH,
}
impl DawnGrowMember {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DawnGrowMember").unwrap();let exd = game_data.read_excel_sheet("DawnGrowMember", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DawnGrowMemberRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DawnGrowMemberRow { columns: row.columns.clone() }
}
}
pub struct DawnGrowMemberRow {
columns: Vec<ColumnData>,
}
impl DawnGrowMemberRow {
pub fn SelectImage(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PortraitImage(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Class(&self) -> &ColumnData {
&self.columns[2]
}
}
