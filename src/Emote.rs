#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Emote {
exd: EXD,
exh: EXH,
}
impl Emote {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Emote")?;let exd = game_data.read_excel_sheet("Emote", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<EmoteRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(EmoteRow { columns })
}
}
pub struct EmoteRow {
columns: Vec<ColumnData>,
}
impl EmoteRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TextCommand(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ActionTimeline(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[5]
}
pub fn LogMessageTargeted(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LogMessageUntargeted(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[8]
}
pub fn EmoteCategory(&self) -> &ColumnData {
&self.columns[9]
}
pub fn EmoteMode(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[15]
}
pub fn HasCancelEmote(&self) -> &ColumnData {
&self.columns[16]
}
pub fn DrawsWeapon(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[18]
}
}
