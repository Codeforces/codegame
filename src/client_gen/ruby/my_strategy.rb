require_relative 'model'

class MyStrategy
    def get_action(player_view)
        raise NotImplementedError
    end
end