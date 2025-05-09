#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MiniGameRA {
exd: EXD,
exh: EXH,
}
impl MiniGameRA {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MiniGameRA").unwrap();let exd = game_data.read_excel_sheet("MiniGameRA", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MiniGameRARow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MiniGameRARow { columns: row.columns.clone() }
}
}
pub struct MiniGameRARow {
columns: Vec<ColumnData>,
}
impl MiniGameRARow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[21]
}
}
