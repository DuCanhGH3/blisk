pub fn clear_errno() {
    errno::set_errno(errno::Errno(0));
}
