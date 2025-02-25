use crate::config::Config;
use indoc::formatdoc;

/// Modify the llmstxt file to add the title, description and long description.
pub(crate) fn add_title_and_descriptions(llmstxt: &mut String, config: &Config) {
    let title = &config.title;
    let description = &config.description;
    let long_description = &config.long_description;

    // Add the title, description and long description to the llmstxt file
    llmstxt.push_str(&formatdoc! {
      "
    # {title}

    > {description}

    {long_description}
    "
    });
}
