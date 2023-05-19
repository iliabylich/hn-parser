mod timeago;

pub(crate) fn add_filters(builder: liquid::ParserBuilder) -> liquid::ParserBuilder {
    builder.filter(timeago::TimeAgo)
}
