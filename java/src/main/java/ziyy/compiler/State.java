package ziyy.compiler;

import java.util.ArrayList;

import static ziyy.value.Value.RESET;

public class State {
    ArrayList<String> tags = new ArrayList<>();
    ArrayList<String> saves = new ArrayList<>();

    public State() {
        tags.add("");
        saves.add(RESET);
    }

    public void push(String tag, String string) {
        int l = saves.size() - 1;
        String s = saves.get(l);
        saves.add(s + string);
        tags.add(tag);
    }

    public String[] pop() {
        int l = saves.size() - 1;
        String a = tags.remove(l);
        String b = saves.remove(l);

        String[] s = {a, b};
        return s;
    }

    public String currentTag() {
        int l = tags.size() - 1;
        return tags.get(l);
    }

    public String currentSave() {
        int l = saves.size() - 1;
        return saves.get(l);
    }
}
