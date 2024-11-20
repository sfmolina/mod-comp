//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  20no24                                             //
//---------------------------------------------------------------//



use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use nalgebra::{DMatrix, DVector};
use rand::Rng;
use crate::data::perceptron_simple::*;
use crate::components::file_uploader::FileUpload;



//-------------------------------------------------------------------
// MAIN


type DataMatrix = DMatrix<f64>;
type WeightsVector = DVector<f64>;
type WeightsHistory = Vec<DVector<f64>>;
type AllCorrect = bool;
type LastEpoch = i32;
fn main_ps(problem: DataMatrix)
-> (DataMatrix, WeightsVector, WeightsHistory, AllCorrect, LastEpoch)
{
    
    // Carga de datos en una matriz
    let data = problem;

    let lr: f64 = 0.5;          // Learning rate
    let max_epochs: i32 = 100;  // Número máximo de épocas
    let mut epoch: i32 = 1;     // Contador de épocas

    let mut w: DVector<f64> = perceptron_weigths_generator(&data);  // Inicialización de pesos

    let mut w_history: Vec<DVector<f64>> = Vec::new();  // Historial de pesos
    w_history.push(w.clone());

    let mut all_correct = false;    // Flag de clasificación correcta de todos los datos

    // Entrenamiento del perceptrón
    while !all_correct && (epoch <= max_epochs) {

        // Iterar sobre todas las filas de la matriz de datos
        // Usamos holdout para entrenar el perceptrón
        for i in 0..data.nrows() {

            // Obtener los valores de input, output y target
            let (input, output, target) = valores_iot(&data, &w, i);

            // Actualizar los pesos si la clasificación es incorrecta
            if output != target {
                w = update_net(w, lr, output, target, input);
                // Guardar los pesos en cada iteración para graficarlos
                w_history.push(w.clone());
            }

        }
        epoch += 1;
        all_correct = check_pattern(&data, &w);
    }

    epoch -= 1; // Restar 1 a epoch para obtener el número de épocas correcto

    (data, w, w_history, all_correct, epoch)

}


//-------------------------------------------------------------------
// AUXILIARY FUNCTIONS


/// Inicializa el vector de pesos con valores aleatorios entre -0.5 y 0.5
fn perceptron_weigths_generator(data: &DMatrix<f64>) -> DVector<f64> {
    let n_inp = data.ncols();
    let mut rng = rand::thread_rng();
    DVector::from_fn(n_inp, |_, _| rng.gen::<f64>() - 0.5)
}


/// Comprueba si el perceptrón clasifica correctamente todos los datos
fn check_pattern(data: &DMatrix<f64>, w: &DVector<f64>) -> bool {

    // Itera sobre todas las filas de la matriz de datos
    // Sale del bucle en cuanto encuentra una mala clasificación
    // Si no encuentra errores, retorna true
    data.row_iter().enumerate().all(|(i, _)| {
        let (_, output, target) = valores_iot(data, w, i);
        output == target
    })

}


type Input = DVector<f64>;
type Output = f64;
type Target = f64;
/// Calcula los valores de input, output y target para un patrón de datos
fn valores_iot(data: &DMatrix<f64>, w: &DVector<f64>, i: usize) -> (Input, Output, Target) {
    
    let row = data.row(i).columns(0, data.ncols() - 1).into_owned();
    let input: DVector<f64> = DVector::from_row_slice(row.as_slice());
    
    let weighted_sum: f64 = input.dot(&w.rows(0, w.len() - 1));
    let output = signo(weighted_sum - w[w.len() - 1]);
    
    let target = data[(i, data.ncols() - 1)];
    
    (input, output, target)
}


/// Actualiza los pesos del perceptrón
fn update_net(mut w: DVector<f64>, lr: f64, output: f64, target: f64, input: DVector<f64>) -> DVector<f64> {
    
    // Agregar -1 al final del input
    let input = input.push(-1.0);

    // Calcular diffW
    let diff_w = lr * (target - output) * input;

    // Actualizar los pesos
    w += diff_w;

    w
}


/// Función de activación
fn signo(x: f64) -> f64 {
    if x >= 0.0 { 1.0 } else { -1.0 }
}


//-------------------------------------------------------------------
// YEW COMPONENT


#[derive(Properties, PartialEq)]
struct PsProps {
    problem: Problem,
    file_data:Vec<Vec<f64>>,
    force_update: usize,
}


#[function_component(PerceptronSimpleComponent)]
pub fn perceptron_simple_component() -> Html {


    // Estado para forzar la actualización
    let force_update = use_state(|| 0_usize);


    // Estado para almacenar el problema seleccionado
    let problem = use_state(|| Problem::And);

    // Función para cambiar el problema
    let set_problem = {
        let problem = problem.clone();
        move |new_problem| problem.set(new_problem)
    };

    // Función para manejar el evento de clic
    let on_click = |new_problem| {
        let set_problem = set_problem.clone();
        let force_update = force_update.clone();
        Callback::from(move |_| {
            set_problem(new_problem);
            force_update.set(*force_update + 1);
        })
    };


    //Para el file upload
    let file_data = use_state(Vec::<Vec<f64>>::new);

    let on_file_upload = {
        let file_data = file_data.clone();
        let set_problem = set_problem.clone();
        let force_update = force_update.clone();
        Callback::from(move |data: Vec<Vec<f64>>| {
            file_data.set(data);
            set_problem(Problem::Csv);
            force_update.set(*force_update + 1);
        })
    };



    html! {
        <div class="container-fluid ps-comp">
            <div class="container d-flex justify-content-center mb-3 top-buttons">
                <h2>{ "Dataset:" }</h2>
                <button class="btn btn-primary custom mx-1" onclick={on_click(Problem::And)}>{ "AND" }</button>
                <button class="btn btn-primary custom mx-1" onclick={on_click(Problem::Or)}>{ "OR" }</button>
                <button class="btn btn-primary custom mx-1" onclick={on_click(Problem::Xor)}>{ "XOR" }</button>
                <button class="btn btn-primary custom mx-1" onclick={on_click(Problem::L5)}>{ "L5" }</button>
                <button class="btn btn-primary custom mx-1" onclick={on_click(Problem::L10)}>{ "L10" }</button>
                <button class="btn btn-primary custom mx-1" onclick={on_click(Problem::L50)}>{ "L50" }</button>
                <button class="btn btn-primary custom mx-1" onclick={on_click(*problem)}>{ "Reload" }</button>
                <FileUpload on_file_upload={on_file_upload} />
            </div>
            <PerceptronSimpleCalculation problem={*problem} file_data={(*file_data).clone()} force_update={*force_update} />
        </div>
    }
}


#[function_component(PerceptronSimpleCalculation)]
fn ps_calculate(props: &PsProps) -> Html {

    let filas: usize;
    let columnas: usize;

    // Seleccionar el problema
    let problem: DMatrix<f64> = match props.problem {
        Problem::And => {
            let mat = and_problem();
            filas = mat.nrows();
            columnas = mat.ncols();
            mat
        },
        Problem::Or => {
            let mat = or_problem();
            filas = mat.nrows();
            columnas = mat.ncols();
            mat
        },
        Problem::Xor => {
            let mat = xor_problem();
            filas = mat.nrows();
            columnas = mat.ncols();
            mat
        },
        Problem::L5 => {
            let mat = l5_problem();
            filas = mat.nrows();
            columnas = mat.ncols();
            mat
        },
        Problem::L10 => {
            let mat = l10_problem();
            filas = mat.nrows();
            columnas = mat.ncols();
            mat
        },
        Problem::L50 => {
            let mat = l50_problem();
            filas = mat.nrows();
            columnas = mat.ncols();
            mat
        },
        Problem::Csv => {
            filas = props.file_data.len();
            columnas = props.file_data[0].len();
            let data = DMatrix::from_row_slice(filas, columnas, &props.file_data.iter().flatten().copied().collect::<Vec<f64>>());
            data
        }
    };


    // Llamar a la función principal del perceptrón con el problema seleccionado
    let result = main_ps(problem);

    
    // Desempaquetar el resultado
    let (data, final_weights, w_history, all_correct, last_epoch) = result;


    // Convertir DMatrix a Vec<Vec<f64>>
    // Esto es necesario para pasar los datos a la función de graficación
    let data_points: Vec<Vec<f64>> = data.row_iter()
    .map(|row| row.iter().cloned().collect())
    .collect();

    // Convertir Vec<DVector<f64>> a Vec<Vec<f64>>
    // Esto es necesario para pasar los pesos en cada iteracion a la función de graficación
    let weights_vec: Vec<Vec<f64>> = w_history.iter()
    .map(|weights| weights.iter().cloned().collect())
    .collect();


    // Graficar
    // Usamos una función de JavaScript de Apache ECharts para graficar
    use_effect(move || {
        let data_points_js = to_value(&data_points).unwrap();
        let weights_js = to_value(&weights_vec).unwrap();
        ps_chart(data_points_js, weights_js);
        || ()
    });


    html! {
        
        <div class="container ps-info">

            <div class="row">
                <div class="col">
                
                    <div class="container d-flex flex-column align-items-center mb-3">
                        <h1>{ format!("Simple Perceptron Results ({})", props.problem) }</h1>
                        <div id="ps-chart"></div>

                        <div class="disclaimer">
                            <p>{"*All calculations and the perceptron process are "}<b>{"computed on the page, nothing has been precomputed."}</b>{" The chart shows the history of weight changes. Reloading the page or changing the dataset will trigger recalculation."}</p>
                        </div>
                    </div>

                </div>
            </div>
            <div class="row">
                <div class="col">
                    <div class="container content">

                        {
                            if all_correct {
                                html! {
                                    <>
                                    <h2>{ "Classification Result ✅" }</h2>
                                    <ul><li>
                                        <p>{ format!("All data classified correctly in {} epochs.", last_epoch) }</p> 
                                    </li></ul>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                    <h2>{ "Classification Result ❌" }</h2>
                                    <ul><li>
                                        <p>{ format!("Some data misclassified after {} epochs.", last_epoch) }</p> 
                                    </li></ul>
                                    </>
                                }
                            }
                        }

                    </div>
                </div>
            </div>

            <div class="row">
                <div class="col">
                    <div class="container content">

                        <h2>{ "Weights" }</h2>
                        <ul>
                            { for final_weights.iter().map(|&weight| html! { <li>{ weight }</li> }) }
                        </ul>

                    </div>
                </div>
            </div>

            <div class="row">
                <div class="col">
                    <div class="container content">

                        <h2>{ format!("Data ({}x{})", filas, columnas) }</h2>
                        <div class="dataset-info">
                            <table class="table table-striped">
                                <thead>
                                    <tr>
                                        { for (0..data.ncols()).map(|i| html! { <th>{ 
                                            if i == data.ncols() - 1 { "Target".to_string() } else { format!("Input {}", i) }
                                        }</th> }) }
                                    </tr>
                                </thead>
                                <tbody>
                                    { for data.row_iter().map(|row| html! {
                                        <tr> 
                                            { for row.iter().map(|&val| html! { <td>{ val }</td> }) }
                                        </tr>
                                    }) }
                                </tbody>
                            </table>
                        </div>

                    </div>
                </div>
            </div>

        </div>
        
    }

}


//-------------------------------------------------------------------
// APACHE ECHARTS


#[wasm_bindgen(inline_js = "

// Función para calcular los puntos de la recta

function calcularRecta(W, limites) {
    const [xMin, xMax] = limites;
    const xVals = [xMin, xMax];
    const yVals = xVals.map(x => (-W[0] * x + W[2]) / W[1]);
    return xVals.map((x, i) => [x, yVals[i]]);
}

function calcularLimites(allPoints) {

    // Inicializar los límites con valores extremos
    let minX = Infinity;
    let maxX = -Infinity;

    // Recorrer todos los puntos para encontrar los límites
    allPoints.forEach(point => {
        if (point[0] < minX) {
            minX = point[0];
        }
        if (point[0] > maxX) {
            maxX = point[0];
        }
    });

    // Añadir un margen de 0.5 a los límites
    minX -= 0.5;
    maxX += 0.5;

    // Redondear los límites al múltiplo más cercano de 0.5
    minX = (Math.round(minX * 2) / 2).toFixed(1);
    maxX = (Math.round(maxX * 2) / 2).toFixed(1);

    return [minX, maxX];
}

export function ps_chart(dataPoints, weights) {

    const limites = calcularLimites(dataPoints);

    
    // Filtrar los puntos en función de su tipo
    const positivePoints = dataPoints.filter(point => point[2] === 1).map(point => [point[0], point[1]]);
    const negativePoints = dataPoints.filter(point => point[2] === -1).map(point => [point[0], point[1]]);


    // Cargar el tema
    fetch('public/themes/roma.json')
        .then(response => response.json())
        .then(theme => {
            echarts.registerTheme('custom_theme', theme);

            // Configuración de la gráfica en ECharts
            const option = {
                color: theme.color, // Establecer la paleta de colores global
                timeline: {
                    axisType: 'category',
                    autoPlay: false,
                    playInterval: 1500,
                    data: weights.map(() => ''),
                    tooltip: {
                        formatter: function(params) {
                            return `Configuración de pesos ${params.dataIndex + 1}`;
                        }
                    },
                    lineStyle: theme.timeline.lineStyle,
                    itemStyle: theme.timeline.itemStyle,
                    progress: {
                        lineStyle: theme.timeline.lineStyle,
                        itemStyle: theme.timeline.itemStyle
                    },
                    controlStyle: theme.timeline.controlStyle,
                    checkpointStyle: theme.timeline.checkpointStyle,
                    label: theme.timeline.label
                },
                options: weights.map((W, index) => ({
                    title: {
                        left: 'center'
                    },
                    xAxis: {
                        type: 'value',
                        min: limites[0],
                        max: limites[1]
                    },
                    yAxis: {
                        type: 'value',
                        min: limites[0],
                        max: limites[1]
                    },
                    series: [
                        {
                            type: 'scatter',
                            data: positivePoints,
                            symbolSize: 12,
                            name: 'Tipo 1',
                            itemStyle: {
                                color: theme.color[12]
                            }
                        },
                        {
                            type: 'scatter',
                            data: negativePoints,
                            symbolSize: 12,
                            name: 'Tipo -1',
                            itemStyle: {
                                color: theme.color[11]
                            }
                        },
                        {
                            type: 'line',
                            data: calcularRecta(W, limites),
                            name: 'Recta',
                            itemStyle: {
                                color: theme.color[5] // Color para los puntos de la línea
                            },
                            lineStyle: theme.line.lineStyle,
                            emphasis: {
                                lineStyle: {
                                    color: theme.color[5] // Mantener el color al hacer hover
                                },
                                itemStyle: {
                                    color: theme.color[5] // Mantener el color al hacer hover
                                }
                            }
                        }
                    ]
                }))
            };

            const chart = echarts.init(document.getElementById('ps-chart'), 'custom_theme');
            window.addEventListener('resize', function() {
                myChart.resize();
            });
            chart.setOption(option);
        });
}
")]
extern "C" {
    fn ps_chart(data_points: JsValue, weights: JsValue);
}
