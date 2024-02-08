use super::AppState;

pub struct WidgetContext{

}

pub trait Widget{
    fn draw(&self, ctx: &WidgetContext, state: &AppState, controller: &mut super::AppControllerHandle);
}