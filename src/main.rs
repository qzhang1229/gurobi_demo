extern crate gurobi;
use gurobi::*;

fn main() {
  let env = Env::new("logfile.log").unwrap();

  // create an empty model which associated with `env`:
  let mut model = env.new_model("model1").unwrap();

  // add decision variables.
  let x1 = model.add_var("x1", Continuous, 0.0, -INFINITY, INFINITY, &[], &[]).unwrap();
  let x2 = model.add_var("x2", Integer, 0.0, -INFINITY, INFINITY, &[], &[]).unwrap();

  // integrate all of the variables into the model.
  model.update().unwrap();

  // add a linear constraint
  model.add_constr("c0", &x1 + 2.0 * &x2, Greater, -14.0).unwrap();
  model.add_constr("c1", -4.0 * &x1 - 1.0 * &x2, Less, -33.0).unwrap();
  model.add_constr("c2", 2.0 * &x1 + &x2, Less, 20.0).unwrap();

  // integrate all of the constraints into the model.
  model.update().unwrap();

  // set the expression of objective function.
  model.set_objective(8.0 * &x1 + &x2, Minimize).unwrap();

  assert_eq!(model.get(attr::IsMIP).unwrap(), 1, "Model is not a MIP.");

  // write constructed model to the file.
  model.write("logfile.lp").unwrap();

  // optimize the model.
  model.optimize().unwrap();
  assert_eq!(model.status().unwrap(), Status::Optimal);

  assert_eq!(model.get(attr::ObjVal).unwrap() , 59.0);

  let val = model.get_values(attr::X, &[x1, x2]).unwrap();
  assert_eq!(val, [6.5, 7.0]);

  let ans = model.get(attr::ObjVal).unwrap();
  println!("{}",ans);
}