import model


class MyStrategy:
    def get_action(self, player_view, debug_interface):
        raise NotImplementedError()

    def debug_update(self, player_view, debug_interface):
        pass
