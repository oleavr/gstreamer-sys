// This file was generated by gir (0fe730d) from gir-files (???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gstreamer_sys as gst;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType, Volatile};

// Enums
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstTagDemuxResult {
    BrokenTag = 0,
    Again = 1,
    Ok = 2,
}
pub const GST_TAG_DEMUX_RESULT_BROKEN_TAG: GstTagDemuxResult = GstTagDemuxResult::BrokenTag;
pub const GST_TAG_DEMUX_RESULT_AGAIN: GstTagDemuxResult = GstTagDemuxResult::Again;
pub const GST_TAG_DEMUX_RESULT_OK: GstTagDemuxResult = GstTagDemuxResult::Ok;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GstTagImageType {
    None = -1,
    Undefined = 0,
    FrontCover = 1,
    BackCover = 2,
    LeafletPage = 3,
    Medium = 4,
    LeadArtist = 5,
    Artist = 6,
    Conductor = 7,
    BandOrchestra = 8,
    Composer = 9,
    Lyricist = 10,
    RecordingLocation = 11,
    DuringRecording = 12,
    DuringPerformance = 13,
    VideoCapture = 14,
    Fish = 15,
    Illustration = 16,
    BandArtistLogo = 17,
    PublisherStudioLogo = 18,
}
pub const GST_TAG_IMAGE_TYPE_NONE: GstTagImageType = GstTagImageType::None;
pub const GST_TAG_IMAGE_TYPE_UNDEFINED: GstTagImageType = GstTagImageType::Undefined;
pub const GST_TAG_IMAGE_TYPE_FRONT_COVER: GstTagImageType = GstTagImageType::FrontCover;
pub const GST_TAG_IMAGE_TYPE_BACK_COVER: GstTagImageType = GstTagImageType::BackCover;
pub const GST_TAG_IMAGE_TYPE_LEAFLET_PAGE: GstTagImageType = GstTagImageType::LeafletPage;
pub const GST_TAG_IMAGE_TYPE_MEDIUM: GstTagImageType = GstTagImageType::Medium;
pub const GST_TAG_IMAGE_TYPE_LEAD_ARTIST: GstTagImageType = GstTagImageType::LeadArtist;
pub const GST_TAG_IMAGE_TYPE_ARTIST: GstTagImageType = GstTagImageType::Artist;
pub const GST_TAG_IMAGE_TYPE_CONDUCTOR: GstTagImageType = GstTagImageType::Conductor;
pub const GST_TAG_IMAGE_TYPE_BAND_ORCHESTRA: GstTagImageType = GstTagImageType::BandOrchestra;
pub const GST_TAG_IMAGE_TYPE_COMPOSER: GstTagImageType = GstTagImageType::Composer;
pub const GST_TAG_IMAGE_TYPE_LYRICIST: GstTagImageType = GstTagImageType::Lyricist;
pub const GST_TAG_IMAGE_TYPE_RECORDING_LOCATION: GstTagImageType = GstTagImageType::RecordingLocation;
pub const GST_TAG_IMAGE_TYPE_DURING_RECORDING: GstTagImageType = GstTagImageType::DuringRecording;
pub const GST_TAG_IMAGE_TYPE_DURING_PERFORMANCE: GstTagImageType = GstTagImageType::DuringPerformance;
pub const GST_TAG_IMAGE_TYPE_VIDEO_CAPTURE: GstTagImageType = GstTagImageType::VideoCapture;
pub const GST_TAG_IMAGE_TYPE_FISH: GstTagImageType = GstTagImageType::Fish;
pub const GST_TAG_IMAGE_TYPE_ILLUSTRATION: GstTagImageType = GstTagImageType::Illustration;
pub const GST_TAG_IMAGE_TYPE_BAND_ARTIST_LOGO: GstTagImageType = GstTagImageType::BandArtistLogo;
pub const GST_TAG_IMAGE_TYPE_PUBLISHER_STUDIO_LOGO: GstTagImageType = GstTagImageType::PublisherStudioLogo;

// Constants
pub const GST_TAG_CAPTURING_CONTRAST: *const c_char = b"capturing-contrast\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_DIGITAL_ZOOM_RATIO: *const c_char = b"capturing-digital-zoom-ratio\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_EXPOSURE_COMPENSATION: *const c_char = b"capturing-exposure-compensation\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_EXPOSURE_MODE: *const c_char = b"capturing-exposure-mode\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_EXPOSURE_PROGRAM: *const c_char = b"capturing-exposure-program\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FLASH_FIRED: *const c_char = b"capturing-flash-fired\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FLASH_MODE: *const c_char = b"capturing-flash-mode\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FOCAL_LENGTH: *const c_char = b"capturing-focal-length\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FOCAL_LENGTH_35_MM: *const c_char = b"capturing-focal-length-35mm\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FOCAL_RATIO: *const c_char = b"capturing-focal-ratio\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_GAIN_ADJUSTMENT: *const c_char = b"capturing-gain-adjustment\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_ISO_SPEED: *const c_char = b"capturing-iso-speed\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_METERING_MODE: *const c_char = b"capturing-metering-mode\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SATURATION: *const c_char = b"capturing-saturation\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SCENE_CAPTURE_TYPE: *const c_char = b"capturing-scene-capture-type\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SHARPNESS: *const c_char = b"capturing-sharpness\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SHUTTER_SPEED: *const c_char = b"capturing-shutter-speed\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SOURCE: *const c_char = b"capturing-source\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_WHITE_BALANCE: *const c_char = b"capturing-white-balance\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_CDDB_DISCID: *const c_char = b"discid\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_CDDB_DISCID_FULL: *const c_char = b"discid-full\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_MUSICBRAINZ_DISCID: *const c_char = b"musicbrainz-discid\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_MUSICBRAINZ_DISCID_FULL: *const c_char = b"musicbrainz-discid-full\0" as *const u8 as *const c_char;
pub const GST_TAG_CMML_CLIP: *const c_char = b"cmml-clip\0" as *const u8 as *const c_char;
pub const GST_TAG_CMML_HEAD: *const c_char = b"cmml-head\0" as *const u8 as *const c_char;
pub const GST_TAG_CMML_STREAM: *const c_char = b"cmml-stream\0" as *const u8 as *const c_char;
pub const GST_TAG_ID3V2_HEADER_SIZE: c_int = 10;
pub const GST_TAG_IMAGE_HORIZONTAL_PPI: *const c_char = b"image-horizontal-ppi\0" as *const u8 as *const c_char;
pub const GST_TAG_IMAGE_VERTICAL_PPI: *const c_char = b"image-vertical-ppi\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICAL_KEY: *const c_char = b"musical-key\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_ALBUMARTISTID: *const c_char = b"musicbrainz-albumartistid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_ALBUMID: *const c_char = b"musicbrainz-albumid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_ARTISTID: *const c_char = b"musicbrainz-artistid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_TRACKID: *const c_char = b"musicbrainz-trackid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_TRMID: *const c_char = b"musicbrainz-trmid\0" as *const u8 as *const c_char;

// Flags
bitflags! {
    #[repr(C)]
    pub struct GstTagLicenseFlags: c_uint {
        const PERMITS_REPRODUCTION = 1;
        const PERMITS_DISTRIBUTION = 2;
        const PERMITS_DERIVATIVE_WORKS = 4;
        const PERMITS_SHARING = 8;
        const REQUIRES_NOTICE = 256;
        const REQUIRES_ATTRIBUTION = 512;
        const REQUIRES_SHARE_ALIKE = 1024;
        const REQUIRES_SOURCE_CODE = 2048;
        const REQUIRES_COPYLEFT = 4096;
        const REQUIRES_LESSER_COPYLEFT = 8192;
        const PROHIBITS_COMMERCIAL_USE = 65536;
        const PROHIBITS_HIGH_INCOME_NATION_USE = 131072;
        const CREATIVE_COMMONS_LICENSE = 16777216;
        const FREE_SOFTWARE_FOUNDATION_LICENSE = 33554432;
    }
}
pub const GST_TAG_LICENSE_PERMITS_REPRODUCTION: GstTagLicenseFlags = GstTagLicenseFlags::PERMITS_REPRODUCTION;
pub const GST_TAG_LICENSE_PERMITS_DISTRIBUTION: GstTagLicenseFlags = GstTagLicenseFlags::PERMITS_DISTRIBUTION;
pub const GST_TAG_LICENSE_PERMITS_DERIVATIVE_WORKS: GstTagLicenseFlags = GstTagLicenseFlags::PERMITS_DERIVATIVE_WORKS;
pub const GST_TAG_LICENSE_PERMITS_SHARING: GstTagLicenseFlags = GstTagLicenseFlags::PERMITS_SHARING;
pub const GST_TAG_LICENSE_REQUIRES_NOTICE: GstTagLicenseFlags = GstTagLicenseFlags::REQUIRES_NOTICE;
pub const GST_TAG_LICENSE_REQUIRES_ATTRIBUTION: GstTagLicenseFlags = GstTagLicenseFlags::REQUIRES_ATTRIBUTION;
pub const GST_TAG_LICENSE_REQUIRES_SHARE_ALIKE: GstTagLicenseFlags = GstTagLicenseFlags::REQUIRES_SHARE_ALIKE;
pub const GST_TAG_LICENSE_REQUIRES_SOURCE_CODE: GstTagLicenseFlags = GstTagLicenseFlags::REQUIRES_SOURCE_CODE;
pub const GST_TAG_LICENSE_REQUIRES_COPYLEFT: GstTagLicenseFlags = GstTagLicenseFlags::REQUIRES_COPYLEFT;
pub const GST_TAG_LICENSE_REQUIRES_LESSER_COPYLEFT: GstTagLicenseFlags = GstTagLicenseFlags::REQUIRES_LESSER_COPYLEFT;
pub const GST_TAG_LICENSE_PROHIBITS_COMMERCIAL_USE: GstTagLicenseFlags = GstTagLicenseFlags::PROHIBITS_COMMERCIAL_USE;
pub const GST_TAG_LICENSE_PROHIBITS_HIGH_INCOME_NATION_USE: GstTagLicenseFlags = GstTagLicenseFlags::PROHIBITS_HIGH_INCOME_NATION_USE;
pub const GST_TAG_LICENSE_CREATIVE_COMMONS_LICENSE: GstTagLicenseFlags = GstTagLicenseFlags::CREATIVE_COMMONS_LICENSE;
pub const GST_TAG_LICENSE_FREE_SOFTWARE_FOUNDATION_LICENSE: GstTagLicenseFlags = GstTagLicenseFlags::FREE_SOFTWARE_FOUNDATION_LICENSE;

// Records
#[repr(C)]
pub struct GstTagDemuxClass {
    pub parent_class: gst::GstElementClass,
    pub min_start_size: c_uint,
    pub min_end_size: c_uint,
    pub identify_tag: Option<unsafe extern "C" fn(*mut GstTagDemux, *mut gst::GstBuffer, gboolean, *mut c_uint) -> gboolean>,
    pub parse_tag: Option<unsafe extern "C" fn(*mut GstTagDemux, *mut gst::GstBuffer, gboolean, *mut c_uint, *mut *mut gst::GstTagList) -> GstTagDemuxResult>,
    pub merge_tags: Option<unsafe extern "C" fn(*mut GstTagDemux, *const gst::GstTagList, *const gst::GstTagList) -> *mut gst::GstTagList>,
    pub reserved: [gpointer; 4],
}

#[repr(C)]
pub struct GstTagDemuxPrivate(c_void);

#[repr(C)]
pub struct GstTagMuxClass {
    pub parent_class: gst::GstElementClass,
    pub render_start_tag: Option<unsafe extern "C" fn(*mut GstTagMux, *const gst::GstTagList) -> *mut gst::GstBuffer>,
    pub render_end_tag: Option<unsafe extern "C" fn(*mut GstTagMux, *const gst::GstTagList) -> *mut gst::GstBuffer>,
    pub _gst_reserved: [gpointer; 4],
}

#[repr(C)]
pub struct GstTagMuxPrivate(c_void);

#[repr(C)]
pub struct GstTagXmpWriterInterface {
    pub parent: gobject::GTypeInterface,
}

// Classes
#[repr(C)]
pub struct GstTagDemux {
    pub element: gst::GstElement,
    pub priv_: *mut GstTagDemuxPrivate,
    pub reserved: [gpointer; 4],
}

#[repr(C)]
pub struct GstTagMux {
    pub element: gst::GstElement,
    pub priv_: *mut GstTagMuxPrivate,
    pub _gst_reserved: [gpointer; 4],
}

// Interfaces
#[repr(C)]
pub struct GstTagXmpWriter(c_void);

extern "C" {

    //=========================================================================
    // GstTagDemuxResult
    //=========================================================================
    pub fn gst_tag_demux_result_get_type() -> GType;

    //=========================================================================
    // GstTagImageType
    //=========================================================================
    pub fn gst_tag_image_type_get_type() -> GType;

    //=========================================================================
    // GstTagLicenseFlags
    //=========================================================================
    pub fn gst_tag_license_flags_get_type() -> GType;

    //=========================================================================
    // GstTagDemux
    //=========================================================================
    pub fn gst_tag_demux_get_type() -> GType;

    //=========================================================================
    // GstTagMux
    //=========================================================================
    pub fn gst_tag_mux_get_type() -> GType;

    //=========================================================================
    // GstTagXmpWriter
    //=========================================================================
    pub fn gst_tag_xmp_writer_get_type() -> GType;
    pub fn gst_tag_xmp_writer_add_all_schemas(config: *mut GstTagXmpWriter);
    pub fn gst_tag_xmp_writer_add_schema(config: *mut GstTagXmpWriter, schema: *const c_char);
    pub fn gst_tag_xmp_writer_has_schema(config: *mut GstTagXmpWriter, schema: *const c_char) -> gboolean;
    pub fn gst_tag_xmp_writer_remove_all_schemas(config: *mut GstTagXmpWriter);
    pub fn gst_tag_xmp_writer_remove_schema(config: *mut GstTagXmpWriter, schema: *const c_char);
    pub fn gst_tag_xmp_writer_tag_list_to_xmp_buffer(config: *mut GstTagXmpWriter, taglist: *const gst::GstTagList, read_only: gboolean) -> *mut gst::GstBuffer;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_tag_check_language_code(lang_code: *const c_char) -> gboolean;
    pub fn gst_tag_freeform_string_to_utf8(data: *const c_char, size: c_int, env_vars: *mut *const c_char) -> *mut c_char;
    pub fn gst_tag_from_id3_tag(id3_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_from_id3_user_tag(type_: *const c_char, id3_user_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_from_vorbis_tag(vorbis_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_get_id3v2_tag_size(buffer: *mut gst::GstBuffer) -> c_uint;
    pub fn gst_tag_get_language_code_iso_639_1(lang_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_language_code_iso_639_2B(lang_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_language_code_iso_639_2T(lang_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_language_codes() -> *mut *mut c_char;
    pub fn gst_tag_get_language_name(language_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_description(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_flags(license_ref: *const c_char) -> GstTagLicenseFlags;
    pub fn gst_tag_get_license_jurisdiction(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_nick(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_title(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_version(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_licenses() -> *mut *mut c_char;
    pub fn gst_tag_id3_genre_count() -> c_uint;
    pub fn gst_tag_id3_genre_get(id: c_uint) -> *const c_char;
    pub fn gst_tag_image_data_to_image_sample(image_data: *const u8, image_data_len: c_uint, image_type: GstTagImageType) -> *mut gst::GstSample;
    pub fn gst_tag_list_add_id3_image(tag_list: *mut gst::GstTagList, image_data: *const u8, image_data_len: c_uint, id3_picture_type: c_uint) -> gboolean;
    pub fn gst_tag_list_from_exif_buffer(buffer: *mut gst::GstBuffer, byte_order: c_int, base_offset: u32) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_exif_buffer_with_tiff_header(buffer: *mut gst::GstBuffer) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_id3v2_tag(buffer: *mut gst::GstBuffer) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_vorbiscomment(data: *const u8, size: size_t, id_data: *const u8, id_data_length: c_uint, vendor_string: *mut *mut c_char) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_vorbiscomment_buffer(buffer: *mut gst::GstBuffer, id_data: *const u8, id_data_length: c_uint, vendor_string: *mut *mut c_char) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_xmp_buffer(buffer: *mut gst::GstBuffer) -> *mut gst::GstTagList;
    pub fn gst_tag_list_new_from_id3v1(data: *const u8) -> *mut gst::GstTagList;
    pub fn gst_tag_list_to_exif_buffer(taglist: *const gst::GstTagList, byte_order: c_int, base_offset: u32) -> *mut gst::GstBuffer;
    pub fn gst_tag_list_to_exif_buffer_with_tiff_header(taglist: *const gst::GstTagList) -> *mut gst::GstBuffer;
    pub fn gst_tag_list_to_vorbiscomment_buffer(list: *const gst::GstTagList, id_data: *const u8, id_data_length: c_uint, vendor_string: *const c_char) -> *mut gst::GstBuffer;
    pub fn gst_tag_list_to_xmp_buffer(list: *const gst::GstTagList, read_only: gboolean, schemas: *mut *const c_char) -> *mut gst::GstBuffer;
    pub fn gst_tag_parse_extended_comment(ext_comment: *const c_char, key: *mut *mut c_char, lang: *mut *mut c_char, value: *mut *mut c_char, fail_if_no_key: gboolean) -> gboolean;
    pub fn gst_tag_register_musicbrainz_tags();
    pub fn gst_tag_to_id3_tag(gst_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_to_vorbis_comments(list: *const gst::GstTagList, tag: *const c_char) -> *mut glib::GList;
    pub fn gst_tag_to_vorbis_tag(gst_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_xmp_list_schemas() -> *mut *const c_char;
    pub fn gst_vorbis_tag_add(list: *mut gst::GstTagList, tag: *const c_char, value: *const c_char);

}
