use deku::prelude::*;

use crate::data_item::{
    AircraftAddress, AircraftIdentification, CalculatedPositionCartesianCorr,
    CalculatedTrackVelocity, CommunicationsCapabilityFlightStatus, DataSourceIdentifier,
    FlightLevelInBinaryRepresentation, MeasuredPositionInPolarCoordinates,
    Mode3ACodeInOctalRepresentation, ModeSMBData, RadarPlotCharacteristics, TargetReportDescriptor,
    TimeOfDay, TrackNumber, TrackStatus,
};
use crate::fspec::{is_fspec, read_fspec};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Cat48 {
    #[deku(reader = "read_fspec(rest)", update = "Self::update_fspec(&self)")]
    pub fspec: Vec<u8>,
    #[deku(skip, cond = "is_fspec(0b1000_0000, fspec, 0)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 0)")]
    pub time_of_day: Option<TimeOfDay>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 0)")]
    pub target_report_descriptor: Option<TargetReportDescriptor>,
    #[deku(skip, cond = "is_fspec(0b1_0000, fspec, 0)")]
    pub measured_position_in_polar_coordinates: Option<MeasuredPositionInPolarCoordinates>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 0)")]
    pub mode_3_a_code_in_octal_representation: Option<Mode3ACodeInOctalRepresentation>,
    #[deku(skip, cond = "is_fspec(0b100, fspec, 0)")]
    pub flight_level_in_binary_repre: Option<FlightLevelInBinaryRepresentation>,
    #[deku(skip, cond = "is_fspec(0b10, fspec, 0)")]
    pub radar_plot_characteristics: Option<RadarPlotCharacteristics>,
    #[deku(skip, cond = "is_fspec(0b1000_0000, fspec, 1)")]
    pub aircraft_address: Option<AircraftAddress>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 1)")]
    pub aircraft_identification: Option<AircraftIdentification>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 1)")]
    pub mode_smb_data: Option<ModeSMBData>,
    #[deku(skip, cond = "is_fspec(0b1_0000, fspec, 1)")]
    pub track_number: Option<TrackNumber>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 1)")]
    pub calculated_position_cartesian_coor: Option<CalculatedPositionCartesianCorr>,
    #[deku(skip, cond = "is_fspec(0b100, fspec, 1)")]
    pub calculated_track_velocity: Option<CalculatedTrackVelocity>,
    #[deku(skip, cond = "is_fspec(0b10, fspec, 1)")]
    pub track_status: Option<TrackStatus>,
    //#[deku(skip, cond = "is_fspec(0b1000_0000, fspec, 2")]
    //pub track_quality = Option<TrackQuality>,
    //#[deku(skip, cond = "is_fspec(0b100_0000, fspec, 2")]
    //pub warning_error_con_target_class = Option<WarningErrorConditionsTargetClass>,
    //#[deku(skip, cond = "is_fspec(0b10_0000, fspec, 2")]
    //pub mode3a_code_confidence_indicator = Option<Mode3ACodeConfidenceIndicator>,
    //#[deku(skip, cond = "is_fspec(0b1_0000, fspec, 2")]
    //pub modec_code_and_confidence_indicator = Option<ModeCCodeAndConfidenceIndicator>,
    //#[deku(skip, cond = "is_fspec(0b1000, fspec, 2")]
    //pub height_measured_by_3d_radar = Option<HeightMeasuredBy3dRadar>,
    //#[deku(skip, cond = "is_fspec(0b100, fspec, 2")]
    //pub radial_doppler_speed = Option<RadialDopplerSpeed>,
    #[deku(skip, cond = "is_fspec(0b10, fspec, 2)")]
    pub communications_capability_flight_status: Option<CommunicationsCapabilityFlightStatus>,
    //TODO fpsec = 3
}

impl Cat48 {
    fn update_fspec(&self) -> Vec<u8> {
        // Start with max fpsec
        let mut fspec = vec![0x00, 0x00, 0x00, 0x00];
        // add Data Items fspecs where they are Some
        if self.data_source_identifier.is_some() {
            fspec[0] = fspec[0] | DataSourceIdentifier::FSPEC;
        }
        if self.time_of_day.is_some() {
            fspec[0] = fspec[0] | TimeOfDay::FSPEC;
        }
        if self.target_report_descriptor.is_some() {
            fspec[0] = fspec[0] | TargetReportDescriptor::FSPEC;
        }
        if self.measured_position_in_polar_coordinates.is_some() {
            fspec[0] = fspec[0] | MeasuredPositionInPolarCoordinates::FSPEC;
        }
        if self.mode_3_a_code_in_octal_representation.is_some() {
            fspec[0] = fspec[0] | Mode3ACodeInOctalRepresentation::FSPEC;
        }
        if self.flight_level_in_binary_repre.is_some() {
            fspec[0] = fspec[0] | FlightLevelInBinaryRepresentation::FSPEC;
        }
        if self.radar_plot_characteristics.is_some() {
            fspec[0] = fspec[0] | RadarPlotCharacteristics::FSPEC;
        }
        if self.aircraft_address.is_some() {
            fspec[1] = fspec[1] | AircraftAddress::FSPEC;
        }
        if self.aircraft_identification.is_some() {
            fspec[1] = fspec[1] | AircraftIdentification::FSPEC;
        }
        if self.mode_smb_data.is_some() {
            fspec[1] = fspec[1] | ModeSMBData::FSPEC;
        }
        if self.track_number.is_some() {
            fspec[1] = fspec[1] | TrackNumber::FSPEC;
        }
        if self.calculated_position_cartesian_coor.is_some() {
            fspec[1] = fspec[1] | CalculatedPositionCartesianCorr::FSPEC;
        }
        if self.calculated_track_velocity.is_some() {
            fspec[1] = fspec[1] | CalculatedTrackVelocity::FSPEC;
        }
        if self.track_status.is_some() {
            fspec[1] = fspec[1] | TrackStatus::FSPEC;
        }
        if self.communications_capability_flight_status.is_some() {
            fspec[2] = fspec[2] | CommunicationsCapabilityFlightStatus::FSPEC;
        }
        // Remove trailing fspecs
        // - find last item in fspec that isn't 00...
        let mut remove_indicies = vec![];
        for (n, f) in fspec.iter().rev().enumerate() {
            if *f != 0x00 {
                break;
            }
            remove_indicies.push(fspec.len() - n);
        }
        for i in &remove_indicies {
            fspec.remove(*i - 1);
        }
        // Add FX bits
        let fspec_len = fspec.len();
        for (n, f) in fspec[..fspec_len].iter_mut().enumerate() {
            if n == fspec_len - 1 {
                break;
            }
            *f = *f | 0b0000_0001
        }
        fspec
    }
}
