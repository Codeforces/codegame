import model


class MyStrategy:
    def get_action(self, player_view):
        raise NotImplementedError()