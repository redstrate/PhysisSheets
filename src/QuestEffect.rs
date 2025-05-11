#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct QuestEffectSheet {
exd: EXD,
exh: EXH,
}
impl QuestEffectSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("QuestEffect")?;let exd = game_data.read_excel_sheet("QuestEffect", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<QuestEffectRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(QuestEffectRow { columns })
}
}
pub struct QuestEffectRow {
columns: Vec<ColumnData>,
}
impl QuestEffectRow {
pub fn UnknownStruct(&self) -> [&ColumnData; 4] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[6]
}
}
