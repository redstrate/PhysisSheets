#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Lobby {
exd: EXD,
exh: EXH,
}
impl Lobby {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Lobby").unwrap();let exd = game_data.read_excel_sheet("Lobby", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LobbyRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LobbyRow { columns: row.columns.clone() }
}
}
pub struct LobbyRow {
columns: Vec<ColumnData>,
}
impl LobbyRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn TYPE(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PARAM(&self) -> &ColumnData {
&self.columns[4]
}
pub fn LINK(&self) -> &ColumnData {
&self.columns[5]
}
}
