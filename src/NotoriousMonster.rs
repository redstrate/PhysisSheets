#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct NotoriousMonster {
exd: EXD,
exh: EXH,
}
impl NotoriousMonster {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("NotoriousMonster").unwrap();let exd = game_data.read_excel_sheet("NotoriousMonster", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> NotoriousMonsterRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
NotoriousMonsterRow { columns: row.columns.clone() }
}
}
pub struct NotoriousMonsterRow {
columns: Vec<ColumnData>,
}
impl NotoriousMonsterRow {
pub fn BNpcName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BNpcBase(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Rank(&self) -> &ColumnData {
&self.columns[3]
}
}
