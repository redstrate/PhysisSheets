#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct DynamicEvent {
exd: EXD,
exh: EXH,
}
impl DynamicEvent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DynamicEvent").unwrap();let exd = game_data.read_excel_sheet("DynamicEvent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DynamicEventRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
DynamicEventRow { columns }
}
}
pub struct DynamicEventRow {
columns: Vec<ColumnData>,
}
impl DynamicEventRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn LGBEventObject(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LGBMapRange(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Announce(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[10]
}
pub fn EventType(&self) -> &ColumnData {
&self.columns[11]
}
pub fn EnemyType(&self) -> &ColumnData {
&self.columns[12]
}
pub fn MaxParticipants(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[15]
}
pub fn SingleBattle(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[17]
}
}
