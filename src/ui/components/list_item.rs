use tui::widgets::ListItem;
#[derive(Clone)]
pub struct ListItemValued<T> {
    list_item: ListItem<'static>,
    value: T,
}
impl<T> ListItemValued<T> {
    pub fn new(list_item: ListItem<'static>, value: T) -> Self {
        Self { list_item, value }
    }
    pub fn get_value(self) -> T {
        self.value
    }
    pub fn get_list_item(self) -> ListItem<'static> {
        self.list_item
    }
}
