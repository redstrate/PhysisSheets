#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct FateSheet {
exd: EXD,
exh: EXH,
}
impl FateSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Fate")?;let exd = game_data.read_excel_sheet("Fate", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<FateRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(FateRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<FateRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct FateRow {
columns: Vec<ColumnData>,
}
impl FateRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Objective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn StatusText(&self) -> [&ColumnData; 3] {
[&self.columns[3],&self.columns[4],&self.columns[5],]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ReqEventItem(&self) -> &ColumnData {
&self.columns[8]
}
pub fn TurnInEventItem(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown2(&self) -> [&ColumnData; 3] {
[&self.columns[10],&self.columns[11],&self.columns[12],]
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
pub fn ObjectiveIcon(&self) -> [&ColumnData; 32] {
[&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],&self.columns[38],&self.columns[39],&self.columns[40],&self.columns[41],&self.columns[42],&self.columns[43],&self.columns[44],&self.columns[45],&self.columns[46],&self.columns[47],]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[48]
}
pub fn EventItem(&self) -> &ColumnData {
&self.columns[49]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[50]
}
pub fn MapIcon(&self) -> &ColumnData {
&self.columns[51]
}
pub fn InactiveMapIcon(&self) -> &ColumnData {
&self.columns[52]
}
pub fn LGBGuardNPCLocation(&self) -> &ColumnData {
&self.columns[53]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[54]
}
pub fn FATEChain(&self) -> &ColumnData {
&self.columns[55]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[56]
}
pub fn ArrayIndex(&self) -> &ColumnData {
&self.columns[57]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[58]
}
pub fn FateRuleEx(&self) -> &ColumnData {
&self.columns[59]
}
pub fn Music(&self) -> &ColumnData {
&self.columns[60]
}
pub fn ScreenImageAccept(&self) -> &ColumnData {
&self.columns[61]
}
pub fn ScreenImageComplete(&self) -> &ColumnData {
&self.columns[62]
}
pub fn ScreenImageFailed(&self) -> &ColumnData {
&self.columns[63]
}
pub fn GivenStatus(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[66]
}
pub fn EurekaFate(&self) -> &ColumnData {
&self.columns[67]
}
pub fn Rule(&self) -> &ColumnData {
&self.columns[68]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[69]
}
pub fn ClassJobLevelMax(&self) -> &ColumnData {
&self.columns[70]
}
pub fn StatusValue(&self) -> [&ColumnData; 3] {
[&self.columns[71],&self.columns[72],&self.columns[73],]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[74]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[75]
}
pub fn SpecialFate(&self) -> &ColumnData {
&self.columns[76]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[77]
}
pub fn AdventEvent(&self) -> &ColumnData {
&self.columns[78]
}
pub fn MoonFaireEvent(&self) -> &ColumnData {
&self.columns[79]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[80]
}
}
