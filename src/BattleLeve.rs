#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BattleLeveSheet {
exd: EXD,
exh: EXH,
}
impl BattleLeveSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BattleLeve")?;let exd = game_data.read_excel_sheet("BattleLeve", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<BattleLeveRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BattleLeveRow { columns })
}
}
pub struct BattleLeveRow {
columns: Vec<ColumnData>,
}
impl BattleLeveRow {
pub fn Time(&self) -> [&ColumnData; 8] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn LeveData(&self) -> [&ColumnData; 8] {
[&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],]
}
pub fn ToDoSequence(&self) -> [&ColumnData; 8] {
[&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Objectives(&self) -> [&ColumnData; 3] {
[&self.columns[25],&self.columns[26],&self.columns[27],]
}
pub fn Help(&self) -> [&ColumnData; 2] {
[&self.columns[28],&self.columns[29],]
}
pub fn Variant(&self) -> &ColumnData {
&self.columns[30]
}
}
