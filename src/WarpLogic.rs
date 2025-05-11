#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct WarpLogicSheet {
exd: EXD,
exh: EXH,
}
impl WarpLogicSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("WarpLogic")?;let exd = game_data.read_excel_sheet("WarpLogic", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<WarpLogicRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WarpLogicRow { columns })
}
}
pub struct WarpLogicRow {
columns: Vec<ColumnData>,
}
impl WarpLogicRow {
pub fn WarpParams(&self) -> [&ColumnData; 10] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],]
}
pub fn Question(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ResponseYes(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ResponseNo(&self) -> &ColumnData {
&self.columns[12]
}
pub fn WarpName(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[14]
}
pub fn CanSkipCutscene(&self) -> &ColumnData {
&self.columns[15]
}
}
