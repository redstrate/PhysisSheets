#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CharaMakeName {
exd: EXD,
exh: EXH,
}
impl CharaMakeName {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CharaMakeName").unwrap();let exd = game_data.read_excel_sheet("CharaMakeName", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CharaMakeNameRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CharaMakeNameRow { columns: row.columns.clone() }
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
