use abuild::project::Project;
fn main() {
    let project: Project =
        Project::builder().directory("project").build().unwrap();
    project.build();
}
