#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DefaultTalk {
exd: EXD,
exh: EXH,
}
impl DefaultTalk {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DefaultTalk").unwrap();let exd = game_data.read_excel_sheet("DefaultTalk", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DefaultTalkRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DefaultTalkRow { columns: row.columns.clone() }
}
}
pub struct DefaultTalkRow {
columns: Vec<ColumnData>,
}
impl DefaultTalkRow {
pub fn DefaultTalkParams(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Text(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[3]
}
}
