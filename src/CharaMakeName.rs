#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CharaMakeNameSheet {
exd: EXD,
exh: EXH,
}
impl CharaMakeNameSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("CharaMakeName")?;let exd = game_data.read_excel_sheet("CharaMakeName", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<CharaMakeNameRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CharaMakeNameRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<CharaMakeNameRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CharaMakeNameRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct CharaMakeNameRow {
columns: Vec<ColumnData>,
}
impl CharaMakeNameRow {
pub fn HyurMidlanderMale(&self) -> &ColumnData {
&self.columns[0]
}
pub fn HyurMidlanderFemale(&self) -> &ColumnData {
&self.columns[1]
}
pub fn HyurMidlanderLastName(&self) -> &ColumnData {
&self.columns[2]
}
pub fn HyurHighlanderMale(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HyurHighlanderFemale(&self) -> &ColumnData {
&self.columns[4]
}
pub fn HyurHighlanderLastName(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ElezenMale(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ElezenFemale(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ElezenWildwoodLastName(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ElezenDuskwightLastName(&self) -> &ColumnData {
&self.columns[9]
}
pub fn MiqoteSunMale(&self) -> &ColumnData {
&self.columns[10]
}
pub fn MiqoteSunFemale(&self) -> &ColumnData {
&self.columns[11]
}
pub fn MiqoteSunMaleLastName(&self) -> &ColumnData {
&self.columns[12]
}
pub fn MiqoteSunFemaleLastName(&self) -> &ColumnData {
&self.columns[13]
}
pub fn MiqoteMoonMale(&self) -> &ColumnData {
&self.columns[14]
}
pub fn MiqoteMoonFemale(&self) -> &ColumnData {
&self.columns[15]
}
pub fn MiqoteMoonLastname(&self) -> &ColumnData {
&self.columns[16]
}
pub fn LalafellPlainsfolkFirstNameStart(&self) -> &ColumnData {
&self.columns[17]
}
pub fn LalafellPlainsfolkLastNameStart(&self) -> &ColumnData {
&self.columns[18]
}
pub fn LalafellPlainsfolkEndOfNames(&self) -> &ColumnData {
&self.columns[19]
}
pub fn LalafellDunesfolkMale(&self) -> &ColumnData {
&self.columns[20]
}
pub fn LalafellDunesfolkMaleLastName(&self) -> &ColumnData {
&self.columns[21]
}
pub fn LalafellDunesfolkFemale(&self) -> &ColumnData {
&self.columns[22]
}
pub fn LalafellDunesfolkFemaleLastName(&self) -> &ColumnData {
&self.columns[23]
}
pub fn RoegadynSeaWolfMale(&self) -> &ColumnData {
&self.columns[24]
}
pub fn RoegadynSeaWolfMaleLastName(&self) -> &ColumnData {
&self.columns[25]
}
pub fn RoegadynSeaWolfFemale(&self) -> &ColumnData {
&self.columns[26]
}
pub fn RoegadynSeaWolfFemaleLastName(&self) -> &ColumnData {
&self.columns[27]
}
pub fn RoegadynHellsguardFirstName(&self) -> &ColumnData {
&self.columns[28]
}
pub fn RoegadynHellsguardMaleLastName(&self) -> &ColumnData {
&self.columns[29]
}
pub fn RoegadynHellsguardFemaleLastName(&self) -> &ColumnData {
&self.columns[30]
}
pub fn AuRaRaenMale(&self) -> &ColumnData {
&self.columns[31]
}
pub fn AuRaRaenFemale(&self) -> &ColumnData {
&self.columns[32]
}
pub fn AuRaRaenLastName(&self) -> &ColumnData {
&self.columns[33]
}
pub fn AuRaXaelaMale(&self) -> &ColumnData {
&self.columns[34]
}
pub fn AuRaXaelaFemale(&self) -> &ColumnData {
&self.columns[35]
}
pub fn AuRaXaelaLastName(&self) -> &ColumnData {
&self.columns[36]
}
pub fn HrothgarHellionsFirstName(&self) -> &ColumnData {
&self.columns[37]
}
pub fn HrothgarHellionsLastName(&self) -> &ColumnData {
&self.columns[38]
}
pub fn HrothgarLostFirstName(&self) -> &ColumnData {
&self.columns[39]
}
pub fn HrothgarLostLastName(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[43]
}
pub fn VieraFirstName(&self) -> &ColumnData {
&self.columns[44]
}
pub fn VieraRavaLastName(&self) -> &ColumnData {
&self.columns[45]
}
pub fn VieraVeenaLastName(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[49]
}
}
