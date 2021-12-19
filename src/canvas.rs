use chrono;
use serde::{Deserialize, Serialize};
use reqwest;

use crate::cli;
use crate::error;
use crate::http;

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    id: u64,
    root_account_id: u64,
    account_id: u64,
    name: String,
    enrollment_term_id: u64,
    uuid: String,
    start_at: Option<chrono::DateTime<chrono::prelude::Local>>,
    grading_standard_id: Option<u64>,
    is_public: bool,
    created_at: chrono::DateTime<chrono::prelude::Local>,
    course_code: String,
    default_view: String,
    license: String,
    // grade_passback_setting: null,
    end_at: Option<chrono::DateTime<chrono::prelude::Local>>,
    public_syllabus: bool,
    public_syllabus_to_auth: bool,
    storage_quota_mb: u64,
    is_public_to_auth_users: bool,
    homeroom_course: bool,
    //course_color: null,
    friendly_name: Option<String>,
    apply_assignment_group_weights: bool,
    calendar: CourseCalendar,
    time_zone: String,
    blueprint: bool,
    template: bool,
    enrollments: Vec<CourseEnrollment>,
    hide_final_grades: bool,
    workflow_state: String,
    restrict_enrollments_to_course_dates: bool,
    overridden_course_visibility: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseEnrollment {
    #[serde(rename="type")]
    type_: String,
    role: String,
    role_id: u64,
    user_id: u64,
    enrollment_state: String,
    limit_privileges_to_course_section: bool,
    associated_user_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseCalendar {
    ics: String,
}

// https://canvas.instructure.com/doc/api/enrollments.html
#[derive(Debug, Deserialize, Serialize)]
pub struct Enrollment {
    // The ID of the enrollment.
    id: u64,
    // The unique id of the course.
    course_id: u64,
    // The SIS Course ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    sis_course_id: String,
    // The Course Integration ID in which the enrollment is associated. This field
    // is only included if the user has permission to view SIS information.
    course_integration_id: String,
    // The unique id of the user's section.
    course_section_id: u64,
    // The Section Integration ID in which the enrollment is associated. This
    // field is only included if the user has permission to view SIS information.
    section_integration_id: String,
    // The SIS Account ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    sis_account_id: String,
    // The SIS Section ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    sis_section_id: String,
    // The SIS User ID in which the enrollment is associated. Only displayed if
    // present. This field is only included if the user has permission to view SIS
    // information.
    sis_user_id: String,
    // The state of the user's enrollment in the course.
    enrollment_state: String,
    // User can only access his or her own course section.
    limit_privileges_to_course_section: bool,
    // The unique identifier for the SIS import. This field is only included if
    // the user has permission to manage SIS information.
    sis_import_id: u64,
    // The unique id of the user's account.
    root_account_id: u64,
    // The enrollment type. One of 'StudentEnrollment', 'TeacherEnrollment',
    // 'TaEnrollment', 'DesignerEnrollment', 'ObserverEnrollment'.
    #[serde(rename="type")]
    type_: String,
    // The unique id of the user.
    user_id: u64,
    // The unique id of the associated user. Will be null unless type is
    // ObserverEnrollment.
    // associated_user_id: null,
    // The enrollment role, for course-level permissions. This field will match
    // `type` if the enrollment role has not been customized.
    role: String,
    // The id of the enrollment role.
    role_id: u64,
    // The created time of the enrollment, in ISO8601 format.
    created_at: chrono::DateTime<chrono::prelude::Local>,
    // The updated time of the enrollment, in ISO8601 format.
    updated_at: chrono::DateTime<chrono::prelude::Local>,
    // The start time of the enrollment, in ISO8601 format.
    start_at: chrono::DateTime<chrono::prelude::Local>,
    // The end time of the enrollment, in ISO8601 format.
    end_at: chrono::DateTime<chrono::prelude::Local>,
    // The last activity time of the user for the enrollment, in ISO8601 format.
    last_activity_at: chrono::DateTime<chrono::prelude::Local>,
    // The last attended date of the user for the enrollment in a course, in ISO8601
    // format.
    last_attended_at: chrono::DateTime<chrono::prelude::Local>,
    // The total activity time of the user for the enrollment, in seconds.
    total_activity_time: u64,
    // The URL to the Canvas web UI page for this course enrollment.
    html_url: String,
    // The URL to the Canvas web UI page containing the grades associated with this
    // enrollment.
    grades: EnrollmentGrade,
    // A description of the user.
    user: User,
    // The user's override grade for the course.
    override_grade: String,
    // The user's override score for the course.
    override_score: f64,
    // The user's current grade in the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins.
    unposted_current_grade: String,
    // The user's final grade for the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins..
    unposted_final_grade: String,
    // The user's current score in the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    unposted_current_score: String,
    // The user's final score for the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    unposted_final_score: String,
    // optional: Indicates whether the course the enrollment belongs to has grading
    // periods set up. (applies only to student enrollments, and only available in
    // course endpoints)
    has_grading_periods: bool,
    // optional: Indicates whether the course the enrollment belongs to has the
    // Display Totals for 'All Grading Periods' feature enabled. (applies only to
    // student enrollments, and only available in course endpoints)
    totals_for_all_grading_periods_option: bool,
    // optional: The name of the currently active grading period, if one exists. If
    // the course the enrollment belongs to does not have grading periods, or if no
    // currently active grading period exists, the value will be null. (applies only
    // to student enrollments, and only available in course endpoints)
    current_grading_period_title: String,
    // optional: The id of the currently active grading period, if one exists. If
    // the course the enrollment belongs to does not have grading periods, or if no
    // currently active grading period exists, the value will be null. (applies only
    // to student enrollments, and only available in course endpoints)
    current_grading_period_id: u64,
    // The user's override grade for the current grading period.
    current_period_override_grade: String,
    // The user's override score for the current grading period.
    current_period_override_score: f64,
    // optional: The student's score in the course for the current grading period,
    // including muted/unposted assignments. Only included if user has permission to
    // view this score, typically teachers, TAs, and admins. If the course the
    // enrollment belongs to does not have grading periods, or if no currently
    // active grading period exists, the value will be null. (applies only to
    // student enrollments, and only available in course endpoints)
    current_period_unposted_current_score: f64,
    // optional: The student's score in the course for the current grading period,
    // including muted/unposted assignments and including ungraded assignments with
    // a score of 0. Only included if user has permission to view this score,
    // typically teachers, TAs, and admins. If the course the enrollment belongs to
    // does not have grading periods, or if no currently active grading period
    // exists, the value will be null. (applies only to student enrollments, and
    // only available in course endpoints)
    current_period_unposted_final_score: f64,
    // optional: The letter grade equivalent of
    // current_period_unposted_current_score, if available. Only included if user
    // has permission to view this grade, typically teachers, TAs, and admins. If
    // the course the enrollment belongs to does not have grading periods, or if no
    // currently active grading period exists, the value will be null. (applies only
    // to student enrollments, and only available in course endpoints)
    current_period_unposted_current_grade: String,
    // optional: The letter grade equivalent of current_period_unposted_final_score,
    // if available. Only included if user has permission to view this grade,
    // typically teachers, TAs, and admins. If the course the enrollment belongs to
    // does not have grading periods, or if no currently active grading period
    // exists, the value will be null. (applies only to student enrollments, and
    // only available in course endpoints)
    current_period_unposted_final_grade: String,
}

// https://canvas.instructure.com/doc/api/enrollments.html
#[derive(Debug, Deserialize, Serialize)]
pub struct EnrollmentGrade {
    // The URL to the Canvas web UI page for the user's grades, if this is a student
    // enrollment.
    html_url: String,
    // The user's current grade in the class. Only included if user has permissions
    // to view this grade.
    current_grade: Option<String>,
    // The user's final grade for the class. Only included if user has permissions
    // to view this grade.
    final_grade: Option<String>,
    // The user's current score in the class. Only included if user has permissions
    // to view this score.
    current_score: Option<String>,
    // The user's final score for the class. Only included if user has permissions
    // to view this score.
    final_score: Option<String>,
    // The total points the user has earned in the class. Only included if user has
    // permissions to view this score and 'current_points' is passed in the
    // request's 'include' parameter.
    current_points: Option<u64>,
    // The user's current grade in the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins.
    unposted_current_grade: Option<String>,
    // The user's final grade for the class including muted/unposted assignments.
    // Only included if user has permissions to view this grade, typically teachers,
    // TAs, and admins..
    unposted_final_grade: Option<String>,
    // The user's current score in the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    unposted_current_score: Option<String>,
    // The user's final score for the class including muted/unposted assignments.
    // Only included if user has permissions to view this score, typically teachers,
    // TAs, and admins..
    unposted_final_score: Option<String>,
    // The total points the user has earned in the class, including muted/unposted
    // assignments. Only included if user has permissions to view this score
    // (typically teachers, TAs, and admins) and 'current_points' is passed in the
    // request's 'include' parameter.
    unposted_current_points: u64
}

// A Canvas user, e.g. a student, teacher, administrator, observer, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    // The ID of the user.
    id: u64,
    // The name of the user.
    name: String,
    // The name of the user that is should be used for sorting groups of users, such
    // as in the gradebook.
    sortable_name: String,
    // The last name of the user.
    last_name: String,
    // The first name of the user.
    first_name: String,
    // A short name the user has selected, for use in conversations or other less
    // formal places through the site.
    short_name: Option<String>,
    // The SIS ID associated with the user.  This field is only included if the user
    // came from a SIS import and has permissions to view SIS information.
    sis_user_id: Option<String>,
    // The id of the SIS import.  This field is only included if the user came from
    // a SIS import and has permissions to manage SIS information.
    sis_import_id: u64,
    // The integration_id associated with the user.  This field is only included if
    // the user came from a SIS import and has permissions to view SIS information.
    integration_id: Option<String>,
    // The unique login id for the user.  This is what the user uses to log in to
    // Canvas.
    login_id: String,
    // If avatars are enabled, this field will be included and contain a url to
    // retrieve the user's avatar.
    avatar_url: Option<String>,
    // Optional: This field can be requested with certain API calls, and will return
    // a list of the users active enrollments. See the List enrollments API for more
    // details about the format of these records.
    enrollments: Option<Vec<CourseEnrollment>>,
    // Optional: This field can be requested with certain API calls, and will return
    // the users primary email address.
    email: Option<String>,
    // Optional: This field can be requested with certain API calls, and will return
    // the users locale in RFC 5646 format.
    locale: Option<String>,
    // Optional: This field is only returned in certain API calls, and will return a
    // timestamp representing the last time the user logged in to canvas.
    last_login: Option<String>,
    // Optional: This field is only returned in certain API calls, and will return
    // the IANA time zone name of the user's preferred timezone.
    time_zone: Option<String>,
    // Optional: The user's bio.
    bio: Option<String>,
}

pub async fn courses<'a>(
    config: &'a cli::CliValid,
) -> Result<Vec<Course>, error::AppError> {
    http::request::<Vec<Course>>(
        config,
        reqwest::Method::GET,
        "https://canvas.instructure.com/api/v1/courses".to_string(),
    ).await
}
