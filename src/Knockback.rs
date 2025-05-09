#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Knockback {
exd: EXD,
exh: EXH,
}
impl Knockback {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Knockback").unwrap();let exd = game_data.read_excel_sheet("Knockback", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> KnockbackRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
KnockbackRow { columns: row.columns.clone() }
}
}
pub struct KnockbackRow {
columns: Vec<ColumnData>,
}
impl KnockbackRow {
pub fn Distance(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Speed(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NearDistance(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Direction(&self) -> &ColumnData {
&self.columns[3]
}
pub fn DirectionArg(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Motion(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CancelMove(&self) -> &ColumnData {
&self.columns[6]
}
}
