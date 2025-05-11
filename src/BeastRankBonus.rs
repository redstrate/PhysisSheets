#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct BeastRankBonusSheet {
exd: EXD,
exh: EXH,
}
impl BeastRankBonusSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("BeastRankBonus")?;let exd = game_data.read_excel_sheet("BeastRankBonus", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<BeastRankBonusRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(BeastRankBonusRow { columns })
}
}
pub struct BeastRankBonusRow {
columns: Vec<ColumnData>,
}
impl BeastRankBonusRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Neutral(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Recognized(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Friendly(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Trusted(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Respected(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Honored(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Sworn(&self) -> &ColumnData {
&self.columns[7]
}
pub fn AlliedBloodsworn(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ItemQuantity(&self) -> [&ColumnData; 8] {
[&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],]
}
}
