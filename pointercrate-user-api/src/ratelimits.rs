use pointercrate_core::localization::tr;
use pointercrate_core::ratelimits;
use std::net::IpAddr;

ratelimits! {
    UserRatelimits {
        registrations[100u32 per 60 per IpAddr] => tr("error-user-ratelimit-registration"),
        soft_registrations[100u32 per 60 per IpAddr] => tr("error-user-ratelimit-soft-registration"),
        login_attempts[100u32 per 60 per IpAddr] => tr("error-user-ratelimit-login"),
    }
}
