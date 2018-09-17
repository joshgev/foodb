module RecipeTypes exposing (
    Category
    ,Verb
    ,Tool
    ,Container
    ,Step
    ,Instruction,
    Recipe)

type alias Recipe = {
    name: String,
    category: Category,
    steps: List Step
    }
    
type alias Category = {
    category_id: Maybe Int,
    name: String
    }
type alias Container = {
    name: String
    }

type alias Tool = {
    name: String
    }

type alias Verb = {
    verb: String    
    }

type alias Step = {
    verb: Verb,
    stopWhen : String,
    stopAfter: Float,
    cotainer: Container,
    tool: Tool
    }

type alias Instruction = {
    steps: List Step
    }